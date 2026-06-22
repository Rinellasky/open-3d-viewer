use std::path::PathBuf;
use std::sync::Mutex;

use tauri::{Emitter, Manager};

/// Holds state we want accessible from Tauri commands.
struct AppState {
  // Mutex<Option<...>> so the JS side can `take` it once and then it's None.
  initial_file: Mutex<Option<String>>,
}

/// Returns the path passed on the command line (e.g. from Windows file
/// associations), if any. Consumed on first call.
#[tauri::command]
fn get_initial_file(state: tauri::State<AppState>) -> Option<String> {
  state.initial_file.lock().ok().and_then(|mut g| g.take())
}

/// Reads a file from disk and returns the raw bytes to the WebView.
/// Returned as `tauri::ipc::Response`, which JS receives as an `ArrayBuffer`
/// (avoids JSON-encoding bytes as an array of numbers).
#[tauri::command]
fn read_file_bytes(path: String) -> Result<tauri::ipc::Response, String> {
  let bytes = std::fs::read(&path).map_err(|e| format!("{}: {}", path, e))?;
  Ok(tauri::ipc::Response::new(bytes))
}

// ---------------------------------------------------------------------------
// Persistent app state (settings + recent files)
// Stored as a single JSON blob in the user's app-data directory:
//   %APPDATA%\com.rinellasky.open3dviewer\state.json   (Windows)
// JS controls the schema; Rust just round-trips the blob.
// ---------------------------------------------------------------------------

const STATE_FILE: &str = "state.json";

fn state_file_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
  let dir = app
    .path()
    .app_data_dir()
    .map_err(|e| format!("Couldn't resolve app data dir: {}", e))?;
  Ok(dir.join(STATE_FILE))
}

#[tauri::command]
fn save_app_state(app: tauri::AppHandle, json: String) -> Result<(), String> {
  let path = state_file_path(&app)?;
  if let Some(parent) = path.parent() {
    std::fs::create_dir_all(parent).map_err(|e| format!("mkdir failed: {}", e))?;
  }
  std::fs::write(&path, json).map_err(|e| format!("write failed: {}", e))
}

#[tauri::command]
fn load_app_state(app: tauri::AppHandle) -> Result<String, String> {
  let path = state_file_path(&app)?;
  if !path.exists() {
    return Ok("{}".to_string());
  }
  std::fs::read_to_string(&path).map_err(|e| format!("read failed: {}", e))
}

// ---------------------------------------------------------------------------
// .usdc support
// Three.js's USDA parser is too limited for real-world USD scenes (chokes on
// primvars, complex shaders, skinning, etc.). Adobe's usdcat ships with the
// usdGltf plugin, which lets us convert .usdc -> .glb directly. We then feed
// the .glb through Three's well-tested GLTFLoader.
// ---------------------------------------------------------------------------

/// Locate `usdcat.exe`. Probes in this order:
///   1. `usdcat` on PATH (works if user did `pip install usd-core` and Scripts is on PATH)
///   2. Known Python install locations (miniconda, anaconda, system Python)
///   3. Adobe Substance 3D Viewer's bundled `usdcat.exe`
fn find_usdcat() -> Result<PathBuf, String> {
  // 1. PATH
  if let Ok(output) = std::process::Command::new("usdcat").arg("--help").output() {
    if output.status.success() {
      return Ok(PathBuf::from("usdcat"));
    }
  }

  // 2. Known Python install locations on Windows
  let mut candidates: Vec<PathBuf> = Vec::new();
  if let Ok(userprofile) = std::env::var("USERPROFILE") {
    candidates.push(PathBuf::from(&userprofile).join("miniconda3\\Scripts\\usdcat.exe"));
    candidates.push(PathBuf::from(&userprofile).join("Anaconda3\\Scripts\\usdcat.exe"));
    candidates.push(PathBuf::from(&userprofile).join("miniforge3\\Scripts\\usdcat.exe"));
  }
  if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
    for v in &["Python311", "Python312", "Python313", "Python310", "Python39"] {
      candidates.push(
        PathBuf::from(&localappdata).join(format!("Programs\\Python\\{}\\Scripts\\usdcat.exe", v)),
      );
    }
  }
  if let Ok(appdata) = std::env::var("APPDATA") {
    for v in &["Python311", "Python312", "Python313", "Python310"] {
      candidates.push(PathBuf::from(&appdata).join(format!("Python\\{}\\Scripts\\usdcat.exe", v)));
    }
  }

  // 3. Adobe Substance 3D Viewer install (last resort, but has usdGltf plugin)
  candidates.push(PathBuf::from(
    "C:\\Program Files\\Adobe\\Adobe Substance 3D Viewer (Beta)\\usdcat.exe",
  ));

  for c in &candidates {
    if c.exists() {
      return Ok(c.clone());
    }
  }

  Err(
    "usdcat not found. Install Pixar USD CLI tools with `pip install usd-core`, \
     or have Adobe Substance 3D Viewer installed."
      .to_string(),
  )
}

/// Returns the resolved usdcat path (for diagnostics / UI).
#[tauri::command]
fn locate_usdcat() -> Result<String, String> {
  find_usdcat().map(|p| p.display().to_string())
}

/// Convert raw USD bytes (any USD variant — .usdc, .usda, .usd) to .glb by
/// shelling out to `usdcat --flatten`. The `usdGltf` plugin (bundled with
/// the Pixar USD distribution and with Adobe's install) does the heavy
/// lifting. Returns the .glb content as an ArrayBuffer in JS, which the
/// front-end then passes to Three.js's GLTFLoader.
///
/// `source_ext` lets the temp file get the right extension for usdcat to
/// recognize the format. Pass "usdc", "usda", or "usd".
#[tauri::command]
fn convert_usd_to_glb(bytes: Vec<u8>, source_ext: String) -> Result<tauri::ipc::Response, String> {
  let usdcat = find_usdcat()?;

  // Sanitize / restrict the extension
  let ext = match source_ext.to_lowercase().as_str() {
    "usdc" => "usdc",
    "usda" => "usda",
    "usd" => "usd",
    other => return Err(format!("Unsupported source extension: {}", other)),
  };

  let temp = std::env::temp_dir();
  let nonce = format!(
    "{}-{}",
    std::process::id(),
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map(|d| d.as_millis())
      .unwrap_or(0)
  );
  let in_path = temp.join(format!("open3dv-{}.{}", nonce, ext));
  let out_path = temp.join(format!("open3dv-{}.glb", nonce));

  std::fs::write(&in_path, &bytes).map_err(|e| format!("Temp write failed: {}", e))?;

  let result = std::process::Command::new(&usdcat)
    .arg("--flatten")
    .arg(&in_path)
    .arg("-o")
    .arg(&out_path)
    .output();

  // Cleanup input regardless
  let _ = std::fs::remove_file(&in_path);

  let result = result.map_err(|e| format!("Failed to spawn usdcat: {}", e))?;
  if !result.status.success() {
    let _ = std::fs::remove_file(&out_path);
    let stderr = String::from_utf8_lossy(&result.stderr).to_string();
    return Err(format!(
      "usdcat exited with {}: {}",
      result.status,
      stderr.trim()
    ));
  }

  let glb_bytes =
    std::fs::read(&out_path).map_err(|e| format!("Couldn't read converted .glb: {}", e))?;
  let _ = std::fs::remove_file(&out_path);

  if glb_bytes.len() < 12 || &glb_bytes[0..4] != b"glTF" {
    return Err(format!(
      "usdcat produced an unexpected output ({} bytes, not a glTF binary). \
       The source may have features the glTF plugin doesn't support.",
      glb_bytes.len()
    ));
  }

  Ok(tauri::ipc::Response::new(glb_bytes))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(initial_file: Option<String>) {
  tauri::Builder::default()
    .manage(AppState {
      initial_file: Mutex::new(initial_file),
    })
    .invoke_handler(tauri::generate_handler![
      get_initial_file,
      read_file_bytes,
      locate_usdcat,
      convert_usd_to_glb,
      save_app_state,
      load_app_state
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Forward OS-level drag-drop to the WebView with the actual file paths
      // (browser drag-drop only gives File objects, no paths — paths are what
      // we need for the Recent Files list and the path-based read pipeline).
      if let Some(main) = app.get_webview_window("main") {
        let emit_target = main.clone();
        main.on_window_event(move |event| {
          if let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event
          {
            let path_strings: Vec<String> =
              paths.iter().map(|p| p.display().to_string()).collect();
            let _ = emit_target.emit("os-drop", path_strings);
          }
        });
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

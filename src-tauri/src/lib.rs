use std::path::PathBuf;
use std::sync::Mutex;

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
// .usdc support: shell-out to Pixar's `usdcat` to convert binary USD to text USD
// so Three.js's USDZLoader (which only parses USDA) can handle it.
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

  // 3. Adobe Substance 3D Viewer install (last resort)
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

/// Convert raw .usdc bytes to flattened .usda bytes by shelling out to usdcat.
/// Returns the converted USDA content as an ArrayBuffer in JS.
#[tauri::command]
fn convert_usdc(bytes: Vec<u8>) -> Result<tauri::ipc::Response, String> {
  let usdcat = find_usdcat()?;

  let temp = std::env::temp_dir();
  let nonce = format!(
    "{}-{}",
    std::process::id(),
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map(|d| d.as_millis())
      .unwrap_or(0)
  );
  let in_path = temp.join(format!("open3dv-{}.usdc", nonce));
  let out_path = temp.join(format!("open3dv-{}.usda", nonce));

  std::fs::write(&in_path, &bytes).map_err(|e| format!("Temp write failed: {}", e))?;

  let result = std::process::Command::new(&usdcat)
    .arg("--flatten")
    .arg(&in_path)
    .arg("-o")
    .arg(&out_path)
    .output();

  // Best-effort cleanup of input
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

  let usda_bytes = std::fs::read(&out_path)
    .map_err(|e| format!("Couldn't read converted USDA: {}", e))?;
  let _ = std::fs::remove_file(&out_path);

  Ok(tauri::ipc::Response::new(usda_bytes))
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
      convert_usdc
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

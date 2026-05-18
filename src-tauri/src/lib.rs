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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(initial_file: Option<String>) {
  tauri::Builder::default()
    .manage(AppState {
      initial_file: Mutex::new(initial_file),
    })
    .invoke_handler(tauri::generate_handler![get_initial_file, read_file_bytes])
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

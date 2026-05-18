// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  // Capture the file path passed by Windows when the app is launched via
  // a file association (double-click) or via command line:
  //   app.exe path\to\model.glb
  let initial_file = std::env::args()
    .nth(1)
    .filter(|s| !s.is_empty() && !s.starts_with("--"));
  app_lib::run(initial_file);
}

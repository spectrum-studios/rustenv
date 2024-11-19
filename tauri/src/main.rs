#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use rustenv_backend::app;

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to pick port");
    tauri::async_runtime::spawn(app(port));

    tauri::Builder::default()
        .manage(port)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

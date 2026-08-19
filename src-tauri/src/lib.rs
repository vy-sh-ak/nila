mod system_tray;

use crate::system_tray::TraySetup;

#[tauri::command]
fn greet(name: &str) -> String {
    format!(
        "Hello, {}! You've been greeted from Rust with hot reload!",
        name
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .setup(init_tray())
        .setup(TraySetup::init)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod app_store;
mod crypto;
mod database;
mod error;
mod system_tray;

use app_store::AppStore;
use crypto::{Crypto, MASTER_KEY_ID};
use database::migrations::plugin_migrations;
use database::DB_URL;
use tauri::Manager;
use system_tray::TraySetup;

pub struct AppState {
    pub crypto: Crypto,
}

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
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(DB_URL, plugin_migrations())
                .build(),
        )
        .setup(|app| {
            TraySetup::init(app)?;

            // Bootstrap the master key used for encrypting secrets at rest
            // (e.g. model API keys). Generated once, persisted in settings.json.
            let handle = app.handle().clone();
            let master_key =
                AppStore::get_or_init_string(&handle, MASTER_KEY_ID, Crypto::generate_key)?;
            app.manage(AppState {
                crypto: Crypto::new(&master_key)?,
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            app_store::store_get,
            app_store::store_set,
            app_store::store_delete,
            database::db_list_migrations,
            database::db_undo_last_migration,
            database::model_factory::model_create,
            database::model_factory::model_list,
            database::model_factory::model_get,
            database::model_factory::model_update,
            database::model_factory::model_delete,
            database::model_factory::model_set_status,
            database::model_factory::model_set_locked,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

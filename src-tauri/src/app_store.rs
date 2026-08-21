use serde_json::Value;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::{Store, StoreExt};

use crate::error::AppError;

const STORE_FILE: &str = "settings.json";

/// Persistent key-value settings backed by the tauri-plugin-store
/// (`settings.json` in the app config directory).
///
/// Every mutation is flushed to disk immediately: the main window only hides
/// on close and the app exits via the tray menu, so we do not rely on
/// save-on-exit.
pub struct AppStore;

impl AppStore {
    pub fn get<R: Runtime>(app: &AppHandle<R>, key: &str) -> Result<Option<Value>, AppError> {
        let store = Self::open(app)?;
        Ok(store.get(key))
    }

    pub fn set<R: Runtime>(app: &AppHandle<R>, key: &str, value: Value) -> Result<(), AppError> {
        let store = Self::open(app)?;
        store.set(key, value);
        store.save().map_err(|e| AppError::Store(e.to_string()))
    }

    pub fn delete<R: Runtime>(app: &AppHandle<R>, key: &str) -> Result<bool, AppError> {
        let store = Self::open(app)?;
        let existed = store.get(key).is_some();
        if existed {
            store.delete(key);
            store.save().map_err(|e| AppError::Store(e.to_string()))?;
        }
        Ok(existed)
    }

    /// Read a string value or fall back to a default, persisting the default
    /// on first use.
    pub fn get_or_init_string<R: Runtime>(
        app: &AppHandle<R>,
        key: &str,
        init: impl FnOnce() -> String,
    ) -> Result<String, AppError> {
        match Self::get(app, key)? {
            Some(Value::String(value)) => Ok(value),
            _ => {
                let value = init();
                Self::set(app, key, Value::String(value.clone()))?;
                Ok(value)
            }
        }
    }

    fn open<R: Runtime>(app: &AppHandle<R>) -> Result<std::sync::Arc<Store<R>>, AppError> {
        app.store(STORE_FILE)
            .map_err(|e| AppError::Store(e.to_string()))
    }
}

#[tauri::command]
pub fn store_get(app: AppHandle, key: String) -> Result<Option<Value>, AppError> {
    AppStore::get(&app, &key)
}

#[tauri::command]
pub fn store_set(app: AppHandle, key: String, value: Value) -> Result<(), AppError> {
    AppStore::set(&app, &key, value)
}

#[tauri::command]
pub fn store_delete(app: AppHandle, key: String) -> Result<bool, AppError> {
    AppStore::delete(&app, &key)
}

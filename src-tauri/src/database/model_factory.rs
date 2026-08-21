use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, State};

use crate::crypto::Crypto;
use crate::database::Database;
use crate::error::AppError;
use crate::AppState;

pub const STATUS_ACTIVE: i64 = 1;
pub const STATUS_INACTIVE: i64 = 0;

/// A model provider connection. `api_key` is decrypted on read; the struct
/// never leaves Rust with ciphertext inside.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Model {
    pub id: i64,
    pub provider: String,
    pub url: String,
    pub api_key: String,
    pub status: i64,
    /// 1 = protected from deletion in the UI, 0 = deletable.
    pub locked: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NewModel {
    pub provider: String,
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelUpdate {
    pub provider: Option<String>,
    pub url: Option<String>,
    pub api_key: Option<String>,
    pub status: Option<i64>,
    pub locked: Option<i64>,
}

/// CRUD factory for the `models` table. Cheap to construct per call
/// (`Pool` is an `Arc` internally).
pub struct ModelFactory {
    pool: Pool<Sqlite>,
}

impl ModelFactory {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    fn flag(value: i64, field: &str) -> Result<i64, AppError> {
        match value {
            STATUS_ACTIVE | STATUS_INACTIVE => Ok(value),
            other => Err(AppError::Validation(format!(
                "`{field}` must be 0 or 1, got {other}"
            ))),
        }
    }

    fn into_decrypted(crypto: &Crypto, mut model: Model) -> Result<Model, AppError> {
        model.api_key = crypto.decrypt(&model.api_key)?;
        Ok(model)
    }

    pub async fn create(&self, crypto: &Crypto, input: NewModel) -> Result<Model, AppError> {
        if input.provider.trim().is_empty() || input.url.trim().is_empty() {
            return Err(AppError::Validation(
                "`provider` and `url` are required".into(),
            ));
        }
        if input.api_key.is_empty() {
            return Err(AppError::Validation("`api_key` is required".into()));
        }
        let api_key = crypto.encrypt(&input.api_key)?;
        let model = sqlx::query_as::<_, Model>(
            "INSERT INTO models (provider, url, api_key) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(input.provider.trim())
        .bind(input.url.trim())
        .bind(api_key)
        .fetch_one(&self.pool)
        .await?;
        Self::into_decrypted(crypto, model)
    }

    pub async fn list(&self, crypto: &Crypto) -> Result<Vec<Model>, AppError> {
        let models = sqlx::query_as::<_, Model>("SELECT * FROM models ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        models
            .into_iter()
            .map(|model| Self::into_decrypted(crypto, model))
            .collect()
    }

    pub async fn get(&self, crypto: &Crypto, id: i64) -> Result<Model, AppError> {
        let model = sqlx::query_as::<_, Model>("SELECT * FROM models WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("model {id}")))?;
        Self::into_decrypted(crypto, model)
    }

    pub async fn update(
        &self,
        crypto: &Crypto,
        id: i64,
        patch: ModelUpdate,
    ) -> Result<Model, AppError> {
        if let Some(status) = patch.status {
            Self::flag(status, "status")?;
        }
        if let Some(locked) = patch.locked {
            Self::flag(locked, "locked")?;
        }

        let encrypted_api_key = match patch.api_key.as_deref() {
            Some("") => return Err(AppError::Validation("`api_key` cannot be empty".into())),
            Some(key) => Some(crypto.encrypt(key)?),
            None => None,
        };

        let mut sets: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(provider) = patch.provider.as_deref() {
            sets.push("provider = ?".into());
            params.push(provider.trim().to_owned());
        }
        if let Some(url) = patch.url.as_deref() {
            sets.push("url = ?".into());
            params.push(url.trim().to_owned());
        }
        if let Some(api_key) = encrypted_api_key {
            sets.push("api_key = ?".into());
            params.push(api_key);
        }

        if sets.is_empty() && patch.status.is_none() && patch.locked.is_none() {
            return Err(AppError::Validation(
                "update requires at least one field".into(),
            ));
        }

        // status/locked are already validated to be 0 or 1, safe to inline.
        if let Some(status) = patch.status {
            sets.push(format!("status = {status}"));
        }
        if let Some(locked) = patch.locked {
            sets.push(format!("locked = {locked}"));
        }
        sets.push("updated_at = datetime('now')".into());

        let sql = format!(
            "UPDATE models SET {} WHERE id = ? RETURNING *",
            sets.join(", ")
        );

        let mut statement = sqlx::query_as::<_, Model>(&sql);
        for param in &params {
            statement = statement.bind(param.as_str());
        }
        let updated = statement
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("model {id}")))?;

        Self::into_decrypted(crypto, updated)
    }

    /// Delete a model. Locked models are rejected unless `force` is set;
    /// returns whether a row was removed.
    pub async fn delete(&self, id: i64, force: bool) -> Result<bool, AppError> {
        let row: Option<i64> = sqlx::query_scalar("SELECT locked FROM models WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            None => Ok(false),
            Some(locked) if locked != 0 && !force => Err(AppError::Validation(format!(
                "model {id} is locked; unlock it first or pass force=true"
            ))),
            Some(_) => {
                let result = sqlx::query("DELETE FROM models WHERE id = ?")
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                Ok(result.rows_affected() > 0)
            },
        }
    }

    pub async fn set_status(&self, crypto: &Crypto, id: i64, status: i64) -> Result<Model, AppError> {
        Self::flag(status, "status")?;
        let model = sqlx::query_as::<_, Model>(
            "UPDATE models SET status = ?, updated_at = datetime('now') WHERE id = ? RETURNING *",
        )
        .bind(status)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("model {id}")))?;
        Self::into_decrypted(crypto, model)
    }

    pub async fn set_locked(&self, crypto: &Crypto, id: i64, locked: i64) -> Result<Model, AppError> {
        Self::flag(locked, "locked")?;
        let model = sqlx::query_as::<_, Model>(
            "UPDATE models SET locked = ?, updated_at = datetime('now') WHERE id = ? RETURNING *",
        )
        .bind(locked)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("model {id}")))?;
        Self::into_decrypted(crypto, model)
    }
}

#[tauri::command]
pub async fn model_create(
    app: AppHandle,
    state: State<'_, AppState>,
    input: NewModel,
) -> Result<Model, AppError> {
    let factory = ModelFactory::new(Database::pool(&app).await?);
    factory.create(&state.crypto, input).await
}

#[tauri::command]
pub async fn model_list(app: AppHandle, state: State<'_, AppState>) -> Result<Vec<Model>, AppError> {
    let factory = ModelFactory::new(Database::pool(&app).await?);
    factory.list(&state.crypto).await
}

#[tauri::command]
pub async fn model_get(
    app: AppHandle,
    state: State<'_, AppState>,
    id: i64,
) -> Result<Model, AppError> {
    let factory = ModelFactory::new(Database::pool(&app).await?);
    factory.get(&state.crypto, id).await
}

#[tauri::command]
pub async fn model_update(
    app: AppHandle,
    state: State<'_, AppState>,
    id: i64,
    input: ModelUpdate,
) -> Result<Model, AppError> {
    let factory = ModelFactory::new(Database::pool(&app).await?);
    factory.update(&state.crypto, id, input).await
}

#[tauri::command]
pub async fn model_delete(app: AppHandle, id: i64, force: Option<bool>) -> Result<bool, AppError> {
    let factory = ModelFactory::new(Database::pool(&app).await?);
    factory.delete(id, force.unwrap_or(false)).await
}

#[tauri::command]
pub async fn model_set_status(
    app: AppHandle,
    state: State<'_, AppState>,
    id: i64,
    status: i64,
) -> Result<Model, AppError> {
    let factory = ModelFactory::new(Database::pool(&app).await?);
    factory.set_status(&state.crypto, id, status).await
}

#[tauri::command]
pub async fn model_set_locked(
    app: AppHandle,
    state: State<'_, AppState>,
    id: i64,
    locked: i64,
) -> Result<Model, AppError> {
    let factory = ModelFactory::new(Database::pool(&app).await?);
    factory.set_locked(&state.crypto, id, locked).await
}

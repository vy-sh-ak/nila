pub mod migrations;
pub mod model_factory;

use sqlx::{Pool, Row, Sqlite};
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{DbInstances, DbPool};

use crate::error::AppError;
use migrations::MIGRATIONS;

/// Connection string handed to tauri-plugin-sql; relative to the app config
/// directory (`com.vyshak.nila`).
pub const DB_URL: &str = "sqlite:nila.db";

/// Access to the SQLite connection pool owned by tauri-plugin-sql.
///
/// The pool is created during plugin setup (see `plugins.sql.preload` in
/// tauri.conf.json), which also applies pending migrations before any command
/// can run.
pub struct Database;

impl Database {
    pub async fn pool(app: &AppHandle) -> Result<Pool<Sqlite>, AppError> {
        // `.inner()` unwraps `State`: its own field `0` is private, so
        // `instances.0` would resolve to that instead of `DbInstances.0`.
        let instances = app.state::<DbInstances>();
        let guard = instances.inner().0.read().await;
        match guard.get(DB_URL) {
            Some(DbPool::Sqlite(pool)) => Ok(pool.clone()),
            _ => Err(AppError::Sql(format!(
                "connection pool for `{DB_URL}` is not initialized"
            ))),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct MigrationInfo {
    pub version: i64,
    pub name: String,
    pub applied: bool,
}

async fn applied_versions(pool: &Pool<Sqlite>) -> Result<Vec<(i64, String)>, AppError> {
    let rows = sqlx::query("SELECT version, description FROM _sqlx_migrations ORDER BY version")
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            (
                row.get::<i64, _>("version"),
                row.get::<String, _>("description"),
            )
        })
        .collect())
}

#[tauri::command]
pub async fn db_list_migrations(app: AppHandle) -> Result<Vec<MigrationInfo>, AppError> {
    let pool = Database::pool(&app).await?;
    let applied = match applied_versions(&pool).await {
        Ok(versions) => versions,
        // The tracking table only exists once the first migration has run.
        Err(err) if err.to_string().contains("no such table") => Vec::new(),
        Err(err) => return Err(err),
    };
    Ok(MIGRATIONS
        .iter()
        .map(|migration| MigrationInfo {
            version: migration.version,
            name: migration.name.to_owned(),
            applied: applied.iter().any(|(version, _)| *version == migration.version),
        })
        .collect())
}

/// Roll back the most recently applied migration using its registered down
/// SQL. Returns the undone version, or `None` when nothing is applied.
///
/// The plugin re-applies the migration on next startup because its tracking
/// row in `_sqlx_migrations` is removed within the same transaction.
///
/// Deliberately synchronous (runs via `block_on`): a sqlx `Transaction` held
/// across awaits inside a `#[tauri::command]` async fn trips rustc's
/// "implementation of Executor is not general enough" false positive
/// (rust-lang/rust#102211), and sync commands carry no `Send` obligation.
#[tauri::command]
pub fn db_undo_last_migration(app: AppHandle) -> Result<Option<i64>, AppError> {
    let pool = tauri::async_runtime::block_on(Database::pool(&app))?;

    let last: Option<i64> =
        match tauri::async_runtime::block_on(
            sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations").fetch_one(&pool),
        ) {
            Ok(version) => version,
            // The tracking table only exists once the first migration has run.
            Err(err) if err.to_string().contains("no such table") => None,
            Err(err) => return Err(err.into()),
        };

    let Some(version) = last else {
        return Ok(None);
    };

    let migration = MIGRATIONS
        .iter()
        .find(|m| m.version == version)
        .ok_or_else(|| {
            AppError::Sql(format!("version {version} has no registered down migration"))
        })?;

    let down_sql = migration.down;
    tauri::async_runtime::block_on(async move {
        let mut tx = pool.begin().await?;
        sqlx::raw_sql(down_sql).execute(&mut *tx).await?;
        sqlx::query("DELETE FROM _sqlx_migrations WHERE version = ?")
            .bind(version)
            .execute(&mut *tx)
            .await?;
        tx.commit().await
    })
    .map_err(AppError::from)?;
    Ok(Some(version))
}

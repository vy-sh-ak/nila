use tauri_plugin_sql::{Migration, MigrationKind};

/// A schema migration with its reversible counterpart.
pub struct MigrationPair {
    pub version: i64,
    pub name: &'static str,
    pub up: &'static str,
    pub down: &'static str,
}

/// Append-only registry. Never edit an entry that has shipped; add a new
/// version instead.
pub const MIGRATIONS: &[MigrationPair] = &[
    MigrationPair {
        version: 1,
        name: "create_models_table",
        up: include_str!("migrations_sql/0001_create_models.up.sql"),
        down: include_str!("migrations_sql/0001_create_models.down.sql"),
    },
    MigrationPair {
        version: 2,
        name: "add_model_metadata",
        up: include_str!("migrations_sql/0002_add_model_metadata.up.sql"),
        down: include_str!("migrations_sql/0002_add_model_metadata.down.sql"),
    },
];

/// Migrations handed to tauri-plugin-sql.
///
/// Both kinds are registered; the plugin filters to `Up` entries when
/// applying (verified against plugins/sql/src/lib.rs), so `Down` SQL stays
/// inert here and is only used by `db_undo_last_migration`.
pub fn plugin_migrations() -> Vec<Migration> {
    MIGRATIONS
        .iter()
        .flat_map(|migration| {
            [
                Migration {
                    version: migration.version,
                    description: migration.name,
                    sql: migration.up,
                    kind: MigrationKind::Up,
                },
                Migration {
                    version: migration.version,
                    description: migration.name,
                    sql: migration.down,
                    kind: MigrationKind::Down,
                },
            ]
        })
        .collect()
}

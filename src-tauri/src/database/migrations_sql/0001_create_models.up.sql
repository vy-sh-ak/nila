CREATE TABLE IF NOT EXISTS models (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    provider    TEXT    NOT NULL,
    url         TEXT    NOT NULL,
    api_key     TEXT    NOT NULL,
    status      INTEGER NOT NULL DEFAULT 1 CHECK (status IN (0, 1)),
    locked      INTEGER NOT NULL DEFAULT 0 CHECK (locked IN (0, 1)),
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_models_provider ON models (provider);

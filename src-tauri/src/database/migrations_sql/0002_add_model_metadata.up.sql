ALTER TABLE models ADD COLUMN name            TEXT NOT NULL DEFAULT '';
ALTER TABLE models ADD COLUMN description     TEXT;
ALTER TABLE models ADD COLUMN logo            TEXT;
ALTER TABLE models ADD COLUMN last_pinged_at  TEXT;

CREATE INDEX IF NOT EXISTS idx_models_name ON models (name);

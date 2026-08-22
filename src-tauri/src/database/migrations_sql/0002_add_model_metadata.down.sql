DROP INDEX IF EXISTS idx_models_name;
ALTER TABLE models DROP COLUMN last_pinged_at;
ALTER TABLE models DROP COLUMN logo;
ALTER TABLE models DROP COLUMN description;
ALTER TABLE models DROP COLUMN name;

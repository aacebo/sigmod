-- Create scorers table
CREATE TABLE IF NOT EXISTS scorers (
    id          UUID        PRIMARY KEY NOT NULL,
    project_id  UUID        NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    type        TEXT        NOT NULL,
    name        TEXT,
    description TEXT,
    options     JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_scorers_project_id  ON scorers(project_id);
CREATE INDEX IF NOT EXISTS idx_scorers_type        ON scorers(type);
CREATE INDEX IF NOT EXISTS idx_scorers_created_at  ON scorers(created_at);
CREATE INDEX IF NOT EXISTS idx_scorers_updated_at  ON scorers(updated_at);

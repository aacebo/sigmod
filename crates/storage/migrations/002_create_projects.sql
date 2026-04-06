-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id           UUID        PRIMARY KEY NOT NULL,
    workspace_id UUID        NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name         TEXT        NOT NULL,
    secret       TEXT        NOT NULL,
    description  TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_projects_workspace_id ON projects(workspace_id);
CREATE INDEX IF NOT EXISTS idx_projects_name         ON projects(name);
CREATE INDEX IF NOT EXISTS idx_projects_created_at   ON projects(created_at);
CREATE INDEX IF NOT EXISTS idx_projects_updated_at   ON projects(updated_at);

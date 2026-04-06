-- Create workspaces table
CREATE TABLE IF NOT EXISTS workspaces (
    id         UUID        PRIMARY KEY NOT NULL,
    name       TEXT        NOT NULL,
    secret     TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_workspaces_name       ON workspaces(name);
CREATE INDEX IF NOT EXISTS idx_workspaces_created_at ON workspaces(created_at);
CREATE INDEX IF NOT EXISTS idx_workspaces_updated_at ON workspaces(updated_at);

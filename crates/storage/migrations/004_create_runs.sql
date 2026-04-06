-- Create runs table
CREATE TABLE IF NOT EXISTS runs (
    id         UUID        PRIMARY KEY NOT NULL,
    scorer_id  UUID        NOT NULL REFERENCES scorers(id) ON DELETE CASCADE,
    status     TEXT        NOT NULL,
    input      TEXT        NOT NULL,
    output     JSONB,
    started_at TIMESTAMPTZ,
    ended_at   TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_runs_scorer_id  ON runs(scorer_id);
CREATE INDEX IF NOT EXISTS idx_runs_status     ON runs(status);
CREATE INDEX IF NOT EXISTS idx_runs_created_at ON runs(created_at);
CREATE INDEX IF NOT EXISTS idx_runs_updated_at ON runs(updated_at);

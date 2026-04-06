-- Add last_run_id to scorers (after runs table exists)
ALTER TABLE scorers ADD COLUMN IF NOT EXISTS last_run_id UUID REFERENCES runs(id) ON DELETE SET NULL;

-- Index
CREATE INDEX IF NOT EXISTS idx_scorers_last_run_id ON scorers(last_run_id);

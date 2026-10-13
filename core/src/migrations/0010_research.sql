
CREATE TABLE IF NOT EXISTS research_runs (
    id UUID PRIMARY KEY,
    objective_id UUID NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    report JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    CONSTRAINT research_runs_status_check
        CHECK (
            status IN (
                'planned',
                'running',
                'completed',
                'partiallycompleted',
                'blocked',
                'failed'
            )
        )
);

CREATE INDEX IF NOT EXISTS idx_research_runs_objective
    ON research_runs (objective_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_research_runs_status
    ON research_runs (status, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_research_runs_created
    ON research_runs (created_at DESC);

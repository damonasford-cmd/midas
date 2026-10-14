CREATE TABLE IF NOT EXISTS forge_jobs (
    id UUID PRIMARY KEY,
    plan_id UUID NOT NULL,
    plan_version INTEGER NOT NULL CHECK (plan_version > 0),
    objective JSONB NOT NULL,
    plan JSONB NOT NULL,

    status TEXT NOT NULL CHECK (
        status IN (
            'proposed',
            'planned',
            'awaiting_authorization',
            'ready',
            'running',
            'blocked',
            'failed',
            'completed',
            'cancelled',
            'superseded'
        )
    ),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,

    CHECK (completed_at IS NULL OR completed_at >= created_at)
);

CREATE INDEX IF NOT EXISTS idx_forge_jobs_status_updated
    ON forge_jobs (status, updated_at DESC);

CREATE TABLE IF NOT EXISTS forge_steps (
    id UUID PRIMARY KEY,
    job_id UUID NOT NULL REFERENCES forge_jobs(id) ON DELETE CASCADE,
    action JSONB NOT NULL,

    status TEXT NOT NULL CHECK (
        status IN (
            'pending',
            'ready',
            'running',
            'blocked',
            'failed',
            'succeeded',
            'skipped'
        )
    ),

    dependencies JSONB NOT NULL DEFAULT '[]'::jsonb,
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    output_summary TEXT,
    error TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    UNIQUE (job_id, id),
    CHECK (
        completed_at IS NULL
        OR started_at IS NULL
        OR completed_at >= started_at
    )
);

CREATE INDEX IF NOT EXISTS idx_forge_steps_job_status
    ON forge_steps (job_id, status);

CREATE TABLE IF NOT EXISTS forge_artifacts (
    id UUID PRIMARY KEY,
    job_id UUID NOT NULL REFERENCES forge_jobs(id) ON DELETE CASCADE,
    step_id UUID NOT NULL,
    artifact JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (job_id, step_id)
        REFERENCES forge_steps (job_id, id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_forge_artifacts_job_created
    ON forge_artifacts (job_id, created_at DESC);

CREATE TABLE IF NOT EXISTS forge_evidence (
    id UUID PRIMARY KEY,
    job_id UUID NOT NULL REFERENCES forge_jobs(id) ON DELETE CASCADE,
    step_id UUID NOT NULL,
    evidence JSONB NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (job_id, step_id)
        REFERENCES forge_steps (job_id, id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_forge_evidence_job_recorded
    ON forge_evidence (job_id, recorded_at DESC);

CREATE TABLE IF NOT EXISTS forge_initiatives (
    id UUID PRIMARY KEY,
    initiative JSONB NOT NULL,
    decision JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assessed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_forge_initiatives_created
    ON forge_initiatives (created_at DESC);

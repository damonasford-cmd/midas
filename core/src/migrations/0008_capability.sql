BEGIN;

CREATE TABLE IF NOT EXISTS capabilities (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    version TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'declared'
        CHECK (state IN (
            'declared',
            'available',
            'degraded',
            'unavailable',
            'suspended',
            'retired'
        )),
    required_permissions JSONB NOT NULL DEFAULT '[]'::jsonb
        CHECK (jsonb_typeof(required_permissions) = 'array'),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
        CHECK (jsonb_typeof(metadata) = 'object'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    evaluation_valid_until TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS capability_dependencies (
    capability_id UUID NOT NULL
        REFERENCES capabilities(id) ON DELETE CASCADE,
    dependency_id UUID NOT NULL
        REFERENCES capabilities(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (capability_id, dependency_id),
    CHECK (capability_id <> dependency_id)
);

CREATE INDEX IF NOT EXISTS idx_capability_dependencies_dependency
    ON capability_dependencies(dependency_id);

CREATE TABLE IF NOT EXISTS capability_evaluations (
    id UUID PRIMARY KEY,
    capability_id UUID NOT NULL
        REFERENCES capabilities(id) ON DELETE CASCADE,
    successful BOOLEAN NOT NULL,
    score DOUBLE PRECISION NOT NULL
        CHECK (score >= 0.0 AND score <= 1.0),
    checks JSONB NOT NULL
        CHECK (jsonb_typeof(checks) = 'array'),
    evidence JSONB NOT NULL
        CHECK (jsonb_typeof(evidence) = 'object'),
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ NOT NULL,
    valid_until TIMESTAMPTZ NOT NULL,
    CHECK (started_at <= completed_at),
    CHECK (completed_at < valid_until)
);

CREATE INDEX IF NOT EXISTS idx_capability_evaluations_latest
    ON capability_evaluations(capability_id, completed_at DESC);

CREATE TABLE IF NOT EXISTS capability_event_outbox (
    id UUID PRIMARY KEY,
    event_type TEXT NOT NULL,
    source TEXT NOT NULL,
    payload JSONB NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    locked_until TIMESTAMPTZ,
    locked_by TEXT,
    published_at TIMESTAMPTZ,
    dead_lettered_at TIMESTAMPTZ,
    last_error TEXT
);

CREATE INDEX IF NOT EXISTS idx_capability_outbox_pending
    ON capability_event_outbox(available_at, created_at)
    WHERE published_at IS NULL AND dead_lettered_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_capability_outbox_lease
    ON capability_event_outbox(locked_until)
    WHERE published_at IS NULL AND dead_lettered_at IS NULL;

COMMIT;

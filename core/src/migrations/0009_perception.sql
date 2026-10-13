BEGIN;

CREATE TABLE IF NOT EXISTS perception_observations (
    id UUID PRIMARY KEY,
    source_id TEXT NOT NULL,
    modality TEXT NOT NULL CHECK (modality IN (
        'text', 'web_page', 'document', 'image', 'audio',
        'video', 'structured_data', 'sensor', 'system_event', 'unknown'
    )),
    content_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    text_content TEXT,
    source_uri TEXT,
    source_record_id TEXT,
    deduplication_key TEXT,
    occurred_at TIMESTAMPTZ NOT NULL,
    acquired_at TIMESTAMPTZ NOT NULL,
    source_confidence DOUBLE PRECISION
        CHECK (
            source_confidence IS NULL OR
            (source_confidence >= 0.0 AND source_confidence <= 1.0)
        ),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
        CHECK (jsonb_typeof(metadata) = 'object'),
    status TEXT NOT NULL CHECK (status IN (
        'received', 'accepted', 'rejected',
        'processed', 'failed', 'archived'
    )),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_perception_source_dedup
    ON perception_observations(source_id, deduplication_key)
    WHERE deduplication_key IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_perception_observations_source_time
    ON perception_observations(source_id, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_perception_observations_modality_time
    ON perception_observations(modality, acquired_at DESC);

CREATE INDEX IF NOT EXISTS idx_perception_observations_status
    ON perception_observations(status, acquired_at);

CREATE TABLE IF NOT EXISTS perception_event_outbox (
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

CREATE INDEX IF NOT EXISTS idx_perception_outbox_pending
    ON perception_event_outbox(available_at, created_at)
    WHERE published_at IS NULL AND dead_lettered_at IS NULL;

COMMIT;

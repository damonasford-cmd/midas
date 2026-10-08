CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS memory_records (
    id UUID PRIMARY KEY,
    modality TEXT NOT NULL,
    content JSONB NOT NULL,
    searchable_text TEXT,
    source TEXT NOT NULL,
    source_reference TEXT,
    occurred_at TIMESTAMPTZ,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verification TEXT NOT NULL,
    sensitivity TEXT NOT NULL,
    confidence REAL,
    embedding VECTOR,
    embedding_model TEXT,
    idempotency_key TEXT UNIQUE,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    content_hash TEXT NOT NULL,
    archived_at TIMESTAMPTZ,
    forgotten_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (confidence IS NULL OR
           (confidence >= 0.0 AND confidence <= 1.0)),
    CHECK (
        (embedding IS NULL AND embedding_model IS NULL)
        OR
        (embedding IS NOT NULL AND embedding_model IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_memory_recorded_at
    ON memory_records (recorded_at DESC);

CREATE INDEX IF NOT EXISTS idx_memory_occurred_at
    ON memory_records (occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_memory_modality
    ON memory_records (modality);

CREATE INDEX IF NOT EXISTS idx_memory_source
    ON memory_records (source);

CREATE INDEX IF NOT EXISTS idx_memory_metadata
    ON memory_records USING GIN (metadata);

CREATE INDEX IF NOT EXISTS idx_memory_searchable_text
    ON memory_records USING GIN (
        to_tsvector('simple', COALESCE(searchable_text, ''))
    );

CREATE INDEX IF NOT EXISTS idx_memory_embedding_model
    ON memory_records (embedding_model)
    WHERE embedding IS NOT NULL;

CREATE TABLE IF NOT EXISTS memory_relations (
    id UUID PRIMARY KEY,
    from_memory UUID NOT NULL
        REFERENCES memory_records(id) ON DELETE CASCADE,
    to_memory UUID NOT NULL
        REFERENCES memory_records(id) ON DELETE CASCADE,
    relation_type TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (from_memory <> to_memory),
    UNIQUE (from_memory, to_memory, relation_type)
);

CREATE INDEX IF NOT EXISTS idx_memory_relations_from
    ON memory_relations (from_memory);

CREATE INDEX IF NOT EXISTS idx_memory_relations_to
    ON memory_relations (to_memory);

CREATE TABLE IF NOT EXISTS memory_event_outbox (
    id UUID PRIMARY KEY,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT
);

CREATE INDEX IF NOT EXISTS idx_memory_outbox_pending
    ON memory_event_outbox (created_at)
    WHERE published_at IS NULL;

CREATE TABLE IF NOT EXISTS memory_lifecycle_log (
    id UUID PRIMARY KEY,
    memory_id UUID NOT NULL,
    operation TEXT NOT NULL,
    reason TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_memory_lifecycle_memory
    ON memory_lifecycle_log (memory_id, occurred_at DESC);

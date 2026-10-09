CREATE TABLE IF NOT EXISTS knowledge_items (
    id UUID PRIMARY KEY,
    subject TEXT NOT NULL,
    predicate TEXT NOT NULL,
    object JSONB NOT NULL,
    statement TEXT NOT NULL,
    source TEXT NOT NULL,
    source_reference TEXT,
    status TEXT NOT NULL CHECK (
        status IN (
            'proposed', 'supported', 'verified',
            'disputed', 'rejected', 'superseded'
        )
    ),
    confidence REAL NOT NULL CHECK (
        confidence >= 0.0 AND confidence <= 1.0
    ),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version >= 1),
    valid_from TIMESTAMPTZ,
    valid_until TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    content_hash TEXT NOT NULL,
    idempotency_key TEXT UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (valid_until IS NULL OR valid_from IS NULL OR valid_until >= valid_from)
);

CREATE INDEX IF NOT EXISTS idx_knowledge_subject
    ON knowledge_items (subject);

CREATE INDEX IF NOT EXISTS idx_knowledge_predicate
    ON knowledge_items (predicate);

CREATE INDEX IF NOT EXISTS idx_knowledge_status
    ON knowledge_items (status);

CREATE INDEX IF NOT EXISTS idx_knowledge_updated
    ON knowledge_items (updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_knowledge_statement_search
    ON knowledge_items USING GIN (to_tsvector('simple', statement));

CREATE INDEX IF NOT EXISTS idx_knowledge_object
    ON knowledge_items USING GIN (object);

CREATE TABLE IF NOT EXISTS knowledge_versions (
    knowledge_id UUID NOT NULL REFERENCES knowledge_items(id) ON DELETE RESTRICT,
    version BIGINT NOT NULL CHECK (version >= 1),
    snapshot JSONB NOT NULL,
    content_hash TEXT NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    change_reason TEXT NOT NULL,
    PRIMARY KEY (knowledge_id, version)
);

CREATE TABLE IF NOT EXISTS knowledge_evidence (
    id UUID PRIMARY KEY,
    knowledge_id UUID NOT NULL REFERENCES knowledge_items(id) ON DELETE RESTRICT,
    polarity TEXT NOT NULL CHECK (
        polarity IN ('supports', 'contradicts', 'neutral')
    ),
    description TEXT NOT NULL,
    source TEXT NOT NULL,
    source_reference TEXT,
    reliability REAL NOT NULL CHECK (
        reliability >= 0.0 AND reliability <= 1.0
    ),
    observed_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    idempotency_key TEXT UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_knowledge_evidence_item
    ON knowledge_evidence (knowledge_id, created_at DESC);

CREATE TABLE IF NOT EXISTS knowledge_uncertainties (
    id UUID PRIMARY KEY,
    knowledge_id UUID NOT NULL REFERENCES knowledge_items(id) ON DELETE RESTRICT,
    kind TEXT NOT NULL CHECK (
        kind IN (
            'missing_evidence', 'conflicting_evidence',
            'ambiguous_meaning', 'outdated_information',
            'source_reliability', 'measurement_error', 'unknown'
        )
    ),
    description TEXT NOT NULL,
    severity REAL NOT NULL CHECK (severity >= 0.0 AND severity <= 1.0),
    resolution_hint TEXT,
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_knowledge_uncertainty_open
    ON knowledge_uncertainties (knowledge_id, created_at DESC)
    WHERE resolved_at IS NULL;

CREATE TABLE IF NOT EXISTS knowledge_relations (
    id UUID PRIMARY KEY,
    from_id UUID NOT NULL REFERENCES knowledge_items(id) ON DELETE RESTRICT,
    to_id UUID NOT NULL REFERENCES knowledge_items(id) ON DELETE RESTRICT,
    relation_type TEXT NOT NULL CHECK (
        relation_type IN (
            'supports', 'contradicts', 'depends_on',
            'derived_from', 'refines', 'supersedes', 'related_to'
        )
    ),
    explanation TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (from_id <> to_id),
    UNIQUE (from_id, to_id, relation_type)
);

CREATE INDEX IF NOT EXISTS idx_knowledge_relations_from
    ON knowledge_relations (from_id);

CREATE INDEX IF NOT EXISTS idx_knowledge_relations_to
    ON knowledge_relations (to_id);

CREATE TABLE IF NOT EXISTS knowledge_event_outbox (
    id UUID PRIMARY KEY,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    idempotency_key TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    locked_until TIMESTAMPTZ,
    locked_by TEXT,
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    published_at TIMESTAMPTZ,
    dead_lettered_at TIMESTAMPTZ,
    last_error TEXT
);

CREATE INDEX IF NOT EXISTS idx_knowledge_outbox_pending
    ON knowledge_event_outbox (available_at, created_at)
    WHERE published_at IS NULL AND dead_lettered_at IS NULL;

ALTER TABLE memory_event_outbox
ADD COLUMN IF NOT EXISTS available_at TIMESTAMPTZ
NOT NULL DEFAULT NOW(),
ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS locked_by TEXT,
ADD COLUMN IF NOT EXISTS dead_lettered_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_memory_outbox_available
ON memory_event_outbox (available_at, created_at)
WHERE published_at IS NULL
AND dead_lettered_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_memory_outbox_lease
ON memory_event_outbox (locked_until)
WHERE published_at IS NULL
AND dead_lettered_at IS NULL;

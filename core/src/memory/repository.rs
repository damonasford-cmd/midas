use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use super::types::{
    MemoryMatch, MemoryModality, MemoryRecord, MemoryRelation,
    MemorySensitivity, MemoryVerification, NewMemory,
    PendingMemoryEvent,
};

pub struct MemoryRepository {
    pool: PgPool,
}

impl MemoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert(
        &self,
        id: Uuid,
        content_hash: &str,
        memory: &NewMemory,
    ) -> Result<MemoryRecord> {
        memory.validate()?;

        let embedding_text = memory.embedding.as_ref().map(format_vector);
        let mut tx = self.pool.begin().await?;

        let inserted = sqlx::query(
            r#"
            INSERT INTO memory_records (
                id, modality, content, searchable_text, source,
                source_reference, occurred_at, verification,
                sensitivity, confidence, embedding, embedding_model,
                idempotency_key, metadata, content_hash
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11::vector, $12, $13, $14, $15
            )
            ON CONFLICT DO NOTHING
            RETURNING id
            "#,
        )
        .bind(id)
        .bind(memory.modality.as_str())
        .bind(&memory.content)
        .bind(&memory.searchable_text)
        .bind(&memory.source)
        .bind(&memory.source_reference)
        .bind(memory.occurred_at)
        .bind(memory.verification.as_str())
        .bind(memory.sensitivity.as_str())
        .bind(memory.confidence)
        .bind(&embedding_text)
        .bind(&memory.embedding_model)
        .bind(&memory.idempotency_key)
        .bind(&memory.metadata)
        .bind(content_hash)
        .fetch_optional(&mut *tx)
        .await?;

        let stored_id = if let Some(row) = inserted {
            let stored_id: Uuid = row.try_get("id")?;

            let event_id = Uuid::new_v4();
            let event_payload = json!({
                "memory_id": stored_id,
                "modality": memory.modality.as_str(),
                "source": memory.source,
                "content_hash": content_hash,
            });

            sqlx::query(
                r#"
                INSERT INTO memory_event_outbox
                    (id, event_type, payload)
                VALUES ($1, 'memory.created', $2)
                "#,
            )
            .bind(event_id)
            .bind(event_payload)
            .execute(&mut *tx)
            .await?;

            stored_id
        } else if let Some(key) = &memory.idempotency_key {
            let existing = sqlx::query(
                r#"
                SELECT id, content_hash
                FROM memory_records
                WHERE idempotency_key = $1
                "#,
            )
            .bind(key)
            .fetch_optional(&mut *tx)
            .await?;

            let Some(row) = existing else {
                bail!("memory insert conflicted without a matching idempotency record");
            };

            let existing_id: Uuid = row.try_get("id")?;
            let existing_hash: String = row.try_get("content_hash")?;

            if existing_hash != content_hash {
                bail!("idempotency key reused with different memory content");
            }

            existing_id
        } else {
            bail!("memory insert conflicted without an idempotency key");
        };

        tx.commit().await?;
        self.get(stored_id).await
    }

    pub async fn get(&self, id: Uuid) -> Result<MemoryRecord> {
        let row = sqlx::query(
            r#"
            SELECT
                id, modality, content, searchable_text, source,
                source_reference, occurred_at, recorded_at,
                verification, sensitivity, confidence,
                embedding::text AS embedding_text,
                embedding_model, metadata, content_hash
            FROM memory_records
            WHERE id = $1
              AND forgotten_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("memory not found or forgotten: {id}"))?;

        decode_memory(&row)
    }

    pub async fn recall_text(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<Vec<MemoryMatch>> {
        if query.trim().is_empty() {
            bail!("text search query cannot be empty");
        }

        if !(1..=500).contains(&limit) {
            bail!("search limit must be between 1 and 500");
        }

        let rows = sqlx::query(
            r#"
            SELECT
                id, modality, content, searchable_text, source,
                source_reference, occurred_at, recorded_at,
                verification, sensitivity, confidence,
                embedding::text AS embedding_text,
                embedding_model, metadata, content_hash,
                ts_rank(
                    to_tsvector('simple', COALESCE(searchable_text, '')),
                    plainto_tsquery('simple', $1)
                ) AS rank
            FROM memory_records
            WHERE forgotten_at IS NULL
              AND archived_at IS NULL
              AND to_tsvector(
                    'simple', COALESCE(searchable_text, '')
                  ) @@ plainto_tsquery('simple', $1)
            ORDER BY rank DESC, recorded_at DESC
            LIMIT $2
            "#,
        )
        .bind(query)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.iter()
            .map(|row| {
                Ok(MemoryMatch {
                    memory: decode_memory(row)?,
                    distance: None,
                })
            })
            .collect()
    }

    pub async fn recall_semantic(
        &self,
        embedding: &[f32],
        embedding_model: &str,
        max_distance: f32,
        limit: i64,
    ) -> Result<Vec<MemoryMatch>> {
        if embedding.is_empty()
            || embedding.iter().any(|value| !value.is_finite())
        {
            bail!("query embedding is empty or contains non-finite values");
        }

        if embedding_model.trim().is_empty() {
            bail!("embedding model cannot be empty");
        }

        if !max_distance.is_finite() || !(0.0..=2.0).contains(&max_distance) {
            bail!("cosine distance must be between 0 and 2");
        }

        if !(1..=500).contains(&limit) {
            bail!("search limit must be between 1 and 500");
        }

        let vector = format_vector(embedding);

        let rows = sqlx::query(
            r#"
            SELECT
                id, modality, content, searchable_text, source,
                source_reference, occurred_at, recorded_at,
                verification, sensitivity, confidence,
                embedding::text AS embedding_text,
                embedding_model, metadata, content_hash,
                embedding <=> $1::vector AS distance
            FROM memory_records
            WHERE forgotten_at IS NULL
              AND archived_at IS NULL
              AND embedding IS NOT NULL
              AND embedding_model = $2
              AND vector_dims(embedding) = $3
              AND embedding <=> $1::vector <= $4
            ORDER BY embedding <=> $1::vector ASC
            LIMIT $5
            "#,
        )
        .bind(&vector)
        .bind(embedding_model)
        .bind(embedding.len() as i32)
        .bind(max_distance)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.iter()
            .map(|row| {
                let distance: f32 = row.try_get("distance")?;

                Ok(MemoryMatch {
                    memory: decode_memory(row)?,
                    distance: Some(distance),
                })
            })
            .collect()
    }

    pub async fn relate(
        &self,
        from: Uuid,
        to: Uuid,
        relation_type: &str,
        metadata: Value,
    ) -> Result<MemoryRelation> {
        if from == to {
            bail!("a memory cannot be related to itself");
        }

        if relation_type.trim().is_empty() {
            bail!("relation type cannot be empty");
        }

        let id = Uuid::new_v4();

        let row = sqlx::query(
            r#"
            INSERT INTO memory_relations (
                id, from_memory, to_memory, relation_type, metadata
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (from_memory, to_memory, relation_type)
            DO UPDATE SET metadata = EXCLUDED.metadata
            RETURNING id, from_memory, to_memory, relation_type,
                      metadata, created_at
            "#,
        )
        .bind(id)
        .bind(from)
        .bind(to)
        .bind(relation_type)
        .bind(metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(MemoryRelation {
            id: row.try_get("id")?,
            from_memory: row.try_get("from_memory")?,
            to_memory: row.try_get("to_memory")?,
            relation_type: row.try_get("relation_type")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
        })
    }

    pub async fn archive(&self, id: Uuid, reason: &str) -> Result<()> {
        self.change_lifecycle(id, "archive", reason).await
    }

    pub async fn forget(&self, id: Uuid, reason: &str) -> Result<()> {
        self.change_lifecycle(id, "forget", reason).await
    }

    async fn change_lifecycle(
        &self,
        id: Uuid,
        operation: &str,
        reason: &str,
    ) -> Result<()> {
        if reason.trim().is_empty() {
            bail!("a lifecycle reason is required");
        }

        let mut tx = self.pool.begin().await?;
        let now = Utc::now();

        let result = match operation {
            "archive" => {
                sqlx::query(
                    r#"
                    UPDATE memory_records
                    SET archived_at = $2, updated_at = $2
                    WHERE id = $1 AND forgotten_at IS NULL
                    "#,
                )
                .bind(id)
                .bind(now)
                .execute(&mut *tx)
                .await?
            }
            "forget" => {
                sqlx::query(
                    r#"
                    UPDATE memory_records
                    SET forgotten_at = $2, updated_at = $2
                    WHERE id = $1 AND forgotten_at IS NULL
                    "#,
                )
                .bind(id)
                .bind(now)
                .execute(&mut *tx)
                .await?
            }
            _ => bail!("unsupported memory lifecycle operation"),
        };

        if result.rows_affected() == 0 {
            bail!("memory not found or already forgotten");
        }

        sqlx::query(
            r#"
            INSERT INTO memory_lifecycle_log
                (id, memory_id, operation, reason)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(id)
        .bind(operation)
        .bind(reason)
        .execute(&mut *tx)
        .await?;

        let payload = json!({
            "memory_id": id,
            "operation": operation,
            "reason": reason,
        });

        sqlx::query(
            r#"
            INSERT INTO memory_event_outbox (id, event_type, payload)
            VALUES ($1, 'memory.lifecycle_changed', $2)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(payload)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn pending_events(
        &self,
        limit: i64,
    ) -> Result<Vec<PendingMemoryEvent>> {
        if !(1..=1000).contains(&limit) {
            bail!("outbox limit must be between 1 and 1000");
        }

        let rows = sqlx::query(
            r#"
            SELECT id, event_type, payload, created_at
            FROM memory_event_outbox
            WHERE published_at IS NULL
            ORDER BY created_at ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.iter()
            .map(|row| {
                Ok(PendingMemoryEvent {
                    id: row.try_get("id")?,
                    event_type: row.try_get("event_type")?,
                    payload: row.try_get("payload")?,
                    created_at: row.try_get("created_at")?,
                })
            })
            .collect()
    }

    pub async fn mark_event_published(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE memory_event_outbox
            SET published_at = NOW(),
                attempts = attempts + 1,
                last_error = NULL
            WHERE id = $1 AND published_at IS NULL
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            bail!("pending outbox event not found: {id}");
        }

        Ok(())
    }

    pub async fn mark_event_failed(
        &self,
        id: Uuid,
        error: &str,
    ) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE memory_event_outbox
            SET attempts = attempts + 1,
                last_error = $2
            WHERE id = $1 AND published_at IS NULL
            "#,
        )
        .bind(id)
        .bind(error)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            bail!("pending outbox event not found: {id}");
        }

        Ok(())
    }
}

fn format_vector(values: &[f32]) -> String {
    let inner = values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",");

    format!("[{inner}]")
}

fn decode_memory(row: &sqlx::postgres::PgRow) -> Result<MemoryRecord> {
    let modality: String = row.try_get("modality")?;
    let verification: String = row.try_get("verification")?;
    let sensitivity: String = row.try_get("sensitivity")?;
    let embedding_text: Option<String> = row.try_get("embedding_text")?;

    Ok(MemoryRecord {
        id: row.try_get("id")?,
        modality: parse_modality(&modality)?,
        content: row.try_get("content")?,
        searchable_text: row.try_get("searchable_text")?,
        source: row.try_get("source")?,
        source_reference: row.try_get("source_reference")?,
        occurred_at: row.try_get("occurred_at")?,
        recorded_at: row.try_get("recorded_at")?,
        verification: parse_verification(&verification)?,
        sensitivity: parse_sensitivity(&sensitivity)?,
        confidence: row.try_get("confidence")?,
        embedding: embedding_text
            .as_deref()
            .map(parse_vector)
            .transpose()?,
        embedding_model: row.try_get("embedding_model")?,
        metadata: row.try_get("metadata")?,
        content_hash: row.try_get("content_hash")?,
    })
}

fn parse_vector(text: &str) -> Result<Vec<f32>> {
    let inner = text
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| anyhow!("invalid vector returned by PostgreSQL"))?;

    if inner.trim().is_empty() {
        return Ok(Vec::new());
    }

    inner
        .split(',')
        .map(|part| {
            let value: f32 = part.trim().parse()?;
            if !value.is_finite() {
                bail!("database vector contains a non-finite value");
            }
            Ok(value)
        })
        .collect()
}

fn parse_modality(value: &str) -> Result<MemoryModality> {
    match value {
        "text" => Ok(MemoryModality::Text),
        "image" => Ok(MemoryModality::Image),
        "audio" => Ok(MemoryModality::Audio),
        "video" => Ok(MemoryModality::Video),
        "file" => Ok(MemoryModality::File),
        "structured_data" => Ok(MemoryModality::StructuredData),
        "world_observation" => Ok(MemoryModality::WorldObservation),
        "action_result" => Ok(MemoryModality::ActionResult),
        "other" => Ok(MemoryModality::Other),
        _ => bail!("unknown memory modality in database: {value}"),
    }
}

fn parse_verification(value: &str) -> Result<MemoryVerification> {
    match value {
        "unverified" => Ok(MemoryVerification::Unverified),
        "observed" => Ok(MemoryVerification::Observed),
        "corroborated" => Ok(MemoryVerification::Corroborated),
        "verified" => Ok(MemoryVerification::Verified),
        "disputed" => Ok(MemoryVerification::Disputed),
        _ => bail!("unknown memory verification state: {value}"),
    }
}

fn parse_sensitivity(value: &str) -> Result<MemorySensitivity> {
    match value {
        "public" => Ok(MemorySensitivity::Public),
        "internal" => Ok(MemorySensitivity::Internal),
        "confidential" => Ok(MemorySensitivity::Confidential),
        "restricted" => Ok(MemorySensitivity::Restricted),
        _ => bail!("unknown memory sensitivity: {value}"),
    }
}

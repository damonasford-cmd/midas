use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use super::{
    modality::{ObservationStatus, PerceptionModality},
    observation::Observation,
    perception_events::PerceptionEvent,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestReceipt {
    pub observation_id: Uuid,
    pub inserted: bool,
}

#[derive(Debug, FromRow)]
struct ObservationRow {
    id: Uuid,
    source_id: String,
    modality: String,
    content_type: String,
    payload: Value,
    text_content: Option<String>,
    source_uri: Option<String>,
    source_record_id: Option<String>,
    deduplication_key: Option<String>,
    occurred_at: DateTime<Utc>,
    acquired_at: DateTime<Utc>,
    source_confidence: Option<f64>,
    metadata: Value,
    status: String,
}

#[derive(Debug, Clone)]
pub struct PerceptionRepository {
    pool: PgPool,
}

impl PerceptionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ingest_batch(
        &self,
        observations: &[Observation],
    ) -> Result<Vec<IngestReceipt>> {
        for observation in observations {
            observation.validate()?;
        }

        let mut tx = self.pool.begin().await?;
        let mut receipts = Vec::with_capacity(observations.len());

        for observation in observations {
            let inserted_id = sqlx::query_scalar::<_, Uuid>(
                r#"
                INSERT INTO perception_observations (
                    id, source_id, modality, content_type, payload,
                    text_content, source_uri, source_record_id,
                    deduplication_key, occurred_at, acquired_at,
                    source_confidence, metadata, status
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6, $7,
                    $8, $9, $10, $11, $12, $13, $14
                )
                ON CONFLICT (source_id, deduplication_key)
                    WHERE deduplication_key IS NOT NULL
                DO NOTHING
                RETURNING id
                "#,
            )
            .bind(observation.id)
            .bind(&observation.source_id)
            .bind(observation.modality.as_str())
            .bind(&observation.content_type)
            .bind(&observation.payload)
            .bind(&observation.text_content)
            .bind(&observation.source_uri)
            .bind(&observation.source_record_id)
            .bind(&observation.deduplication_key)
            .bind(observation.occurred_at)
            .bind(observation.acquired_at)
            .bind(observation.source_confidence)
            .bind(&observation.metadata)
            .bind(observation.status.as_str())
            .fetch_optional(&mut *tx)
            .await
            .context("Could not persist perception observation")?;

            if let Some(id) = inserted_id {
                let event = PerceptionEvent::observation_ingested(observation);

                sqlx::query(
                    r#"
                    INSERT INTO perception_event_outbox (
                        id, event_type, source, payload, occurred_at
                    )
                    VALUES ($1, $2, $3, $4, $5)
                    "#,
                )
                .bind(event.id)
                .bind(&event.event_type)
                .bind(&event.source)
                .bind(&event.payload)
                .bind(event.occurred_at)
                .execute(&mut *tx)
                .await
                .context("Could not persist perception event")?;

                receipts.push(IngestReceipt {
                    observation_id: id,
                    inserted: true,
                });
            } else {
                let existing_id = sqlx::query_scalar::<_, Uuid>(
                    r#"
                    SELECT id
                    FROM perception_observations
                    WHERE source_id = $1
                      AND deduplication_key = $2
                    "#,
                )
                .bind(&observation.source_id)
                .bind(
                    observation
                        .deduplication_key
                        .as_ref()
                        .context("Conflict occurred without a deduplication key")?,
                )
                .fetch_one(&mut *tx)
                .await
                .context("Could not retrieve existing deduplicated observation")?;

                receipts.push(IngestReceipt {
                    observation_id: existing_id,
                    inserted: false,
                });
            }
        }

        tx.commit().await?;
        Ok(receipts)
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Observation>> {
        let row = sqlx::query_as::<_, ObservationRow>(
            r#"
            SELECT id, source_id, modality, content_type, payload,
                   text_content, source_uri, source_record_id,
                   deduplication_key, occurred_at, acquired_at,
                   source_confidence, metadata, status
            FROM perception_observations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(Self::row_to_observation).transpose()
    }

    pub async fn list_recent(
        &self,
        limit: i64,
        before: Option<DateTime<Utc>>,
    ) -> Result<Vec<Observation>> {
        anyhow::ensure!((1..=1000).contains(&limit), "Limit must be 1..=1000");

        let rows = sqlx::query_as::<_, ObservationRow>(
            r#"
            SELECT id, source_id, modality, content_type, payload,
                   text_content, source_uri, source_record_id,
                   deduplication_key, occurred_at, acquired_at,
                   source_confidence, metadata, status
            FROM perception_observations
            WHERE ($1::timestamptz IS NULL OR acquired_at < $1)
            ORDER BY acquired_at DESC, id DESC
            LIMIT $2
            "#,
        )
        .bind(before)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(Self::row_to_observation).collect()
    }

    pub async fn set_status(
        &self,
        id: Uuid,
        next: ObservationStatus,
    ) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE perception_observations
            SET status = $2
            WHERE id = $1
              AND status <> 'archived'
            "#,
        )
        .bind(id)
        .bind(next.as_str())
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() == 1)
    }

    fn row_to_observation(row: ObservationRow) -> Result<Observation> {
        Ok(Observation {
            id: row.id,
            source_id: row.source_id,
            modality: PerceptionModality::try_from(row.modality.as_str())
                .map_err(anyhow::Error::msg)?,
            content_type: row.content_type,
            payload: row.payload,
            text_content: row.text_content,
            source_uri: row.source_uri,
            source_record_id: row.source_record_id,
            deduplication_key: row.deduplication_key,
            occurred_at: row.occurred_at,
            acquired_at: row.acquired_at,
            source_confidence: row.source_confidence,
            metadata: row.metadata,
            status: ObservationStatus::try_from(row.status.as_str())
                .map_err(anyhow::Error::msg)?,
        })
    }
}

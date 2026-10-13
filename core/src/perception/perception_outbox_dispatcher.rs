use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PendingPerceptionEvent {
    pub id: Uuid,
    pub event_type: String,
    pub source: String,
    pub payload: Value,
    pub occurred_at: chrono::DateTime<Utc>,
    pub attempts: i32,
}

#[async_trait]
pub trait PerceptionEventPublisher: Send + Sync {
    async fn publish(&self, event: &PendingPerceptionEvent) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PerceptionOutboxConfig {
    pub batch_size: i64,
    pub lease_seconds: i64,
    pub max_attempts: i32,
    pub retry_base_seconds: i64,
    pub retry_max_seconds: i64,
}

impl Default for PerceptionOutboxConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            lease_seconds: 60,
            max_attempts: 12,
            retry_base_seconds: 2,
            retry_max_seconds: 900,
        }
    }
}

impl PerceptionOutboxConfig {
    fn validate(&self) -> Result<()> {
        anyhow::ensure!(self.batch_size > 0, "batch_size must be positive");
        anyhow::ensure!(self.lease_seconds > 0, "lease_seconds must be positive");
        anyhow::ensure!(self.max_attempts > 0, "max_attempts must be positive");
        anyhow::ensure!(
            self.retry_base_seconds > 0,
            "retry_base_seconds must be positive"
        );
        anyhow::ensure!(
            self.retry_max_seconds >= self.retry_base_seconds,
            "retry_max_seconds must be >= retry_base_seconds"
        );
        Ok(())
    }
}

#[derive(Debug, FromRow)]
struct ClaimedEvent {
    id: Uuid,
    event_type: String,
    source: String,
    payload: Value,
    occurred_at: chrono::DateTime<Utc>,
    attempts: i32,
}

pub struct PerceptionOutboxDispatcher<P> {
    pool: PgPool,
    publisher: P,
    config: PerceptionOutboxConfig,
    worker_id: String,
}

impl<P> PerceptionOutboxDispatcher<P>
where
    P: PerceptionEventPublisher,
{
    pub fn new(
        pool: PgPool,
        publisher: P,
        config: PerceptionOutboxConfig,
        worker_id: String,
    ) -> Result<Self> {
        config.validate()?;
        anyhow::ensure!(!worker_id.trim().is_empty(), "worker_id cannot be empty");

        Ok(Self {
            pool,
            publisher,
            config,
            worker_id,
        })
    }

    pub async fn dispatch_once(&self) -> Result<usize> {
        let events = self.claim_batch().await?;
        let count = events.len();

        for row in events {
            let event = PendingPerceptionEvent {
                id: row.id,
                event_type: row.event_type,
                source: row.source,
                payload: row.payload,
                occurred_at: row.occurred_at,
                attempts: row.attempts,
            };

            match self.publisher.publish(&event).await {
                Ok(()) => self.mark_published(event.id).await?,
                Err(error) => {
                    self.record_failure(
                        event.id,
                        event.attempts,
                        &error.to_string(),
                    )
                    .await?;
                }
            }
        }

        Ok(count)
    }

    async fn claim_batch(&self) -> Result<Vec<ClaimedEvent>> {
        let mut tx = self.pool.begin().await?;
        let lease_until = Utc::now() + Duration::seconds(self.config.lease_seconds);

        let events = sqlx::query_as::<_, ClaimedEvent>(
            r#"
            WITH candidates AS (
                SELECT id
                FROM perception_event_outbox
                WHERE published_at IS NULL
                  AND dead_lettered_at IS NULL
                  AND available_at <= NOW()
                  AND (locked_until IS NULL OR locked_until < NOW())
                ORDER BY created_at, id
                FOR UPDATE SKIP LOCKED
                LIMIT $1
            )
            UPDATE perception_event_outbox AS o
            SET locked_until = $2,
                locked_by = $3,
                attempts = o.attempts + 1
            FROM candidates
            WHERE o.id = candidates.id
            RETURNING o.id, o.event_type, o.source, o.payload,
                      o.occurred_at, o.attempts
            "#,
        )
        .bind(self.config.batch_size)
        .bind(lease_until)
        .bind(&self.worker_id)
        .fetch_all(&mut *tx)
        .await
        .context("Could not claim perception outbox events")?;

        tx.commit().await?;
        Ok(events)
    }

    async fn mark_published(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE perception_event_outbox
            SET published_at = NOW(),
                locked_until = NULL,
                locked_by = NULL,
                last_error = NULL
            WHERE id = $1
              AND locked_by = $2
              AND published_at IS NULL
            "#,
        )
        .bind(id)
        .bind(&self.worker_id)
        .execute(&self.pool)
        .await?;

        anyhow::ensure!(
            result.rows_affected() == 1,
            "Outbox lease lost before acknowledgement for event {id}"
        );

        Ok(())
    }

    async fn record_failure(
        &self,
        id: Uuid,
        attempts: i32,
        message: &str,
    ) -> Result<()> {
        let message: String = message.chars().take(4000).collect();

        if attempts >= self.config.max_attempts {
            sqlx::query(
                r#"
                UPDATE perception_event_outbox
                SET dead_lettered_at = NOW(),
                    locked_until = NULL,
                    locked_by = NULL,
                    last_error = $3
                WHERE id = $1
                  AND locked_by = $2
                  AND published_at IS NULL
                "#,
            )
            .bind(id)
            .bind(&self.worker_id)
            .bind(message)
            .execute(&self.pool)
            .await?;

            return Ok(());
        }

        let exponent = attempts.saturating_sub(1).clamp(0, 20) as u32;
        let multiplier = 1_i64.checked_shl(exponent).unwrap_or(i64::MAX);
        let delay = self
            .config
            .retry_base_seconds
            .saturating_mul(multiplier)
            .min(self.config.retry_max_seconds);
        let available_at = Utc::now() + Duration::seconds(delay);

        sqlx::query(
            r#"
            UPDATE perception_event_outbox
            SET available_at = $3,
                locked_until = NULL,
                locked_by = NULL,
                last_error = $4
            WHERE id = $1
              AND locked_by = $2
              AND published_at IS NULL
            "#,
        )
        .bind(id)
        .bind(&self.worker_id)
        .bind(available_at)
        .bind(message)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

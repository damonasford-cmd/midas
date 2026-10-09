use std::{sync::Arc, time::Duration};

use anyhow::{bail, Context, Result};
use serde_json::json;
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use crate::event_fabric::{EventEnvelope, EventFabric};

pub struct KnowledgeOutboxDispatcher {
    pool: PgPool,
    fabric: Arc<EventFabric>,
    lease_seconds: i64,
    max_attempts: i32,
    batch_size: i64,
}

#[derive(Debug, Clone, Default)]
pub struct KnowledgeDispatchReport {
    pub claimed: usize,
    pub published: usize,
    pub retried: usize,
    pub dead_lettered: usize,
}

struct OutboxItem {
    id: Uuid,
    event_type: String,
    payload: serde_json::Value,
    attempts: i32,
}

impl KnowledgeOutboxDispatcher {
    pub fn new(
        pool: PgPool,
        fabric: Arc<EventFabric>,
        lease_seconds: i64,
        max_attempts: i32,
        batch_size: i64,
    ) -> Result<Self> {
        if lease_seconds <= 0 {
            bail!("lease_seconds must be positive");
        }
        if max_attempts <= 0 {
            bail!("max_attempts must be positive");
        }
        if !(1..=1000).contains(&batch_size) {
            bail!("batch_size must be between 1 and 1000");
        }

        Ok(Self {
            pool,
            fabric,
            lease_seconds,
            max_attempts,
            batch_size,
        })
    }

    pub async fn run_once(&self) -> Result<KnowledgeDispatchReport> {
        let worker = Uuid::new_v4().to_string();
        let items = self.claim_batch(&worker).await?;

        let mut report = KnowledgeDispatchReport {
            claimed: items.len(),
            ..Default::default()
        };

        for item in items {
            match self.publish_one(&item).await {
                Ok(()) => {
                    self.mark_published(item.id, &worker)
                        .await
                        .with_context(|| {
                            format!(
                                "event {} was submitted but its outbox acknowledgement failed",
                                item.id
                            )
                        })?;

                    report.published += 1;
                }
                Err(error) => {
                    let message = format!("{error:#}");
                    let is_dead = self
                        .record_failure(&item, &worker, &message)
                        .await?;

                    if is_dead {
                        report.dead_lettered += 1;
                    } else {
                        report.retried += 1;
                    }
                }
            }
        }

        Ok(report)
    }

    async fn claim_batch(&self, worker: &str) -> Result<Vec<OutboxItem>> {
        let rows = sqlx::query(
            r#"
            WITH candidates AS (
                SELECT id
                FROM knowledge_event_outbox
                WHERE published_at IS NULL
                  AND dead_lettered_at IS NULL
                  AND available_at <= NOW()
                  AND (
                      locked_until IS NULL
                      OR locked_until < NOW()
                  )
                ORDER BY available_at, created_at
                FOR UPDATE SKIP LOCKED
                LIMIT $1
            )
            UPDATE knowledge_event_outbox AS outbox
            SET locked_by = $2,
                locked_until = NOW() + ($3 * INTERVAL '1 second'),
                attempts = outbox.attempts + 1
            FROM candidates
            WHERE outbox.id = candidates.id
            RETURNING outbox.id, outbox.event_type,
                      outbox.payload, outbox.attempts
            "#,
        )
        .bind(self.batch_size)
        .bind(worker)
        .bind(self.lease_seconds)
        .fetch_all(&self.pool)
        .await
        .context("failed to claim knowledge outbox events")?;

        rows.into_iter()
            .map(|row| {
                Ok(OutboxItem {
                    id: row.try_get("id")?,
                    event_type: row.try_get("event_type")?,
                    payload: row.try_get("payload")?,
                    attempts: row.try_get("attempts")?,
                })
            })
            .collect()
    }

    async fn publish_one(&self, item: &OutboxItem) -> Result<()> {
        let envelope = EventEnvelope::new(
            item.event_type.clone(),
            "midas.knowledge",
            json!({
                "outbox_id": item.id,
                "data": item.payload,
            }),
        )?
        .with_idempotency_key(format!("knowledge-outbox:{}", item.id))?;

        self.fabric
            .publish(envelope)
            .await
            .with_context(|| {
                format!("failed to publish knowledge event {}", item.id)
            })?;

        Ok(())
    }

    async fn mark_published(&self, id: Uuid, worker: &str) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE knowledge_event_outbox
            SET published_at = NOW(),
                locked_by = NULL,
                locked_until = NULL,
                last_error = NULL
            WHERE id = $1
              AND locked_by = $2
              AND published_at IS NULL
              AND dead_lettered_at IS NULL
            "#,
        )
        .bind(id)
        .bind(worker)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 1 {
            bail!("outbox event acknowledgement failed: lease lost or event changed");
        }

        Ok(())
    }

    async fn record_failure(
        &self,
        item: &OutboxItem,
        worker: &str,
        error: &str,
    ) -> Result<bool> {
        let dead_lettered = item.attempts >= self.max_attempts;

        let delay_seconds = 2_i64
            .saturating_pow(item.attempts.saturating_sub(1).clamp(0, 8) as u32)
            .min(300);

        let result = if dead_lettered {
            sqlx::query(
                r#"
                UPDATE knowledge_event_outbox
                SET dead_lettered_at = NOW(),
                    locked_by = NULL,
                    locked_until = NULL,
                    last_error = $3
                WHERE id = $1
                  AND locked_by = $2
                  AND published_at IS NULL
                  AND dead_lettered_at IS NULL
                "#,
            )
            .bind(item.id)
            .bind(worker)
            .bind(error)
            .execute(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                UPDATE knowledge_event_outbox
                SET available_at = NOW() + ($3 * INTERVAL '1 second'),
                    locked_by = NULL,
                    locked_until = NULL,
                    last_error = $4
                WHERE id = $1
                  AND locked_by = $2
                  AND published_at IS NULL
                  AND dead_lettered_at IS NULL
                "#,
            )
            .bind(item.id)
            .bind(worker)
            .bind(delay_seconds)
            .bind(error)
            .execute(&self.pool)
            .await?
        };

        if result.rows_affected() != 1 {
            bail!("failed to record event failure: lease lost or event changed");
        }

        Ok(dead_lettered)
    }

    pub fn poll_interval() -> Duration {
        Duration::from_secs(2)
    }
}

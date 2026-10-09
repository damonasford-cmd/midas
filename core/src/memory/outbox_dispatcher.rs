
use std::{sync::Arc, time::Duration};

use anyhow::{bail, Context, Result};
use serde_json::json;
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use crate::event_fabric::{EventEnvelope, EventFabric};

/// Transmet les événements mémorisés en base vers l'Event Fabric.
///
/// Un seul événement logique est publié pour chaque identifiant d'outbox.
/// La livraison est au moins une fois : les consommateurs doivent également
/// traiter les clés d'idempotence de manière cohérente.
pub struct MemoryOutboxDispatcher {
    pool: PgPool,
    fabric: Arc<EventFabric>,
    lease_seconds: i64,
    max_attempts: i32,
    batch_size: i64,
}

#[derive(Debug, Clone, Default)]
pub struct DispatchReport {
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

impl MemoryOutboxDispatcher {
    pub fn new(
        pool: PgPool,
        fabric: Arc<EventFabric>,
        lease_seconds: i64,
        max_attempts: i32,
        batch_size: i64,
    ) -> Result<Self> {
        if lease_seconds <= 0 {
            bail!("outbox lease_seconds must be positive");
        }
        if max_attempts <= 0 {
            bail!("outbox max_attempts must be positive");
        }
        if batch_size <= 0 || batch_size > 1000 {
            bail!("outbox batch_size must be between 1 and 1000");
        }

        Ok(Self {
            pool,
            fabric,
            lease_seconds,
            max_attempts,
            batch_size,
        })
    }

    /// Traite un lot. À appeler périodiquement depuis le runtime de MIDAS.
    pub async fn run_once(&self) -> Result<DispatchReport> {
        let worker_token = Uuid::new_v4().to_string();
        let items = self.claim_batch(&worker_token).await?;

        let mut report = DispatchReport {
            claimed: items.len(),
            ..DispatchReport::default()
        };

        for item in items {
            let result = self.publish_one(&item).await;

            match result {
                Ok(()) => {
                    self.mark_published(item.id, &worker_token)
                        .await
                        .with_context(|| {
                            format!(
                                "published event {} but could not confirm outbox state",
                                item.id
                            )
                        })?;

                    report.published += 1;
                }
                Err(error) => {
                    let error_message = format!("{error:#}");

                    let dead_lettered = self
                        .record_failure(
                            &item,
                            &worker_token,
                            &error_message,
                        )
                        .await?;

                    if dead_lettered {
                        report.dead_lettered += 1;
                    } else {
                        report.retried += 1;
                    }
                }
            }
        }

        Ok(report)
    }

    async fn claim_batch(&self, worker_token: &str) -> Result<Vec<OutboxItem>> {
        // Le CTE et l'UPDATE forment une seule instruction atomique.
        // SKIP LOCKED permet à plusieurs processus de se partager le travail.
        let rows = sqlx::query(
            r#"
            WITH candidates AS (
                SELECT id
                FROM memory_event_outbox
                WHERE published_at IS NULL
                  AND dead_lettered_at IS NULL
                  AND available_at <= NOW()
                  AND (
                      locked_until IS NULL
                      OR locked_until < NOW()
                  )
                ORDER BY created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT $1
            )
            UPDATE memory_event_outbox AS outbox
            SET locked_until =
                    NOW() + ($2 * INTERVAL '1 second'),
                locked_by = $3,
                attempts = outbox.attempts + 1
            FROM candidates
            WHERE outbox.id = candidates.id
            RETURNING
                outbox.id,
                outbox.event_type,
                outbox.payload,
                outbox.attempts
            "#,
        )
        .bind(self.batch_size)
        .bind(self.lease_seconds)
        .bind(worker_token)
        .fetch_all(&self.pool)
        .await
        .context("failed to claim memory outbox events")?;

        rows.iter()
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
        // Le payload reste stable entre les tentatives : l'Event Fabric
        // peut donc reconnaître la même opération via sa clé d'idempotence.
        let payload = json!({
            "outbox_id": item.id,
            "data": item.payload,
        });

        let event = EventEnvelope::new(
            item.event_type.clone(),
            "midas.memory",
            payload,
        )?
        .with_idempotency_key(format!("memory-outbox:{}", item.id))?;

        self.fabric
            .publish(event)
            .await
            .with_context(|| {
                format!(
                    "Event Fabric rejected outbox event {}",
                    item.id
                )
            })?;

        Ok(())
    }

    async fn mark_published(
        &self,
        id: Uuid,
        worker_token: &str,
    ) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE memory_event_outbox
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
        .bind(worker_token)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 1 {
            bail!(
                "outbox lease lost or event already acknowledged: {id}"
            );
        }

        Ok(())
    }

    async fn record_failure(
        &self,
        item: &OutboxItem,
        worker_token: &str,
        error: &str,
    ) -> Result<bool> {
        let dead_lettered = item.attempts >= self.max_attempts;

        // Exponential backoff borné à cinq minutes.
        let exponent = item.attempts.saturating_sub(1).min(8) as u32;
        let delay_seconds = (2_i64.pow(exponent)).min(300);

        let result = if dead_lettered {
            sqlx::query(
                r#"
                UPDATE memory_event_outbox
                SET dead_lettered_at = NOW(),
                    locked_until = NULL,
                    locked_by = NULL,
                    last_error = $3
                WHERE id = $1
                  AND locked_by = $2
                  AND published_at IS NULL
                  AND dead_lettered_at IS NULL
                "#,
            )
            .bind(item.id)
            .bind(worker_token)
            .bind(error)
            .execute(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                UPDATE memory_event_outbox
                SET available_at =
                        NOW() + ($3 * INTERVAL '1 second'),
                    locked_until = NULL,
                    locked_by = NULL,
                    last_error = $4
                WHERE id = $1
                  AND locked_by = $2
                  AND published_at IS NULL
                  AND dead_lettered_at IS NULL
                "#,
            )
            .bind(item.id)
            .bind(worker_token)
            .bind(delay_seconds)
            .bind(error)
            .execute(&self.pool)
            .await?
        };

        if result.rows_affected() != 1 {
            bail!(
                "could not record failure; outbox lease lost for {}",
                item.id
            );
        }

        Ok(dead_lettered)
    }

    /// Intervalle recommandé entre deux appels run_once().
    pub fn poll_interval() -> Duration {
        Duration::from_secs(2)
    }
}

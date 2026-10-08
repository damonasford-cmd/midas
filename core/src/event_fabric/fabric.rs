use std::{sync::Arc, time::Duration};

use anyhow::{bail, Result};
use uuid::Uuid;

use super::{
    envelope::EventEnvelope,
    store::{
        AppendReceipt, ClaimedEvent, DeadLetter, EventStore,
        SequencedEvent,
    },
};

#[derive(Debug, Clone)]
pub struct EventFabricConfig {
    pub max_attempts: u32,
    pub lease_seconds: u64,
    pub retry_base_seconds: u64,
    pub retry_max_seconds: u64,
    pub default_batch_size: usize,
}

impl Default for EventFabricConfig {
    fn default() -> Self {
        Self {
            max_attempts: 8,
            lease_seconds: 30,
            retry_base_seconds: 2,
            retry_max_seconds: 300,
            default_batch_size: 64,
        }
    }
}

pub struct EventFabric {
    store: Arc<dyn EventStore>,
    config: EventFabricConfig,
}

impl EventFabric {
    pub fn new(
        store: Arc<dyn EventStore>,
        config: EventFabricConfig,
    ) -> Result<Self> {
        if config.max_attempts == 0 {
            bail!("max_attempts must be greater than zero");
        }
        if config.lease_seconds == 0 {
            bail!("lease_seconds must be greater than zero");
        }
        if config.default_batch_size == 0 {
            bail!("default_batch_size must be greater than zero");
        }
        if config.retry_max_seconds < config.retry_base_seconds {
            bail!("retry_max_seconds must be >= retry_base_seconds");
        }

        Ok(Self { store, config })
    }

    /// Enregistre l'événement dans le magasin configuré.
    /// La durabilité dépend de l'adaptateur EventStore utilisé.
    pub async fn publish(
        &self,
        event: EventEnvelope,
    ) -> Result<AppendReceipt> {
        event.validate()?;
        self.store.append(event).await
    }

    pub async fn claim(
        &self,
        consumer: &str,
        batch_size: Option<usize>,
    ) -> Result<Vec<ClaimedEvent>> {
        let size = batch_size.unwrap_or(self.config.default_batch_size);

        if size == 0 {
            bail!("batch size must be greater than zero");
        }

        self.store
            .claim_batch(
                consumer,
                size,
                Duration::from_secs(self.config.lease_seconds),
            )
            .await
    }

    /// L'accusé de réception est lié à cette tentative précise.
    pub async fn acknowledge(&self, claimed: &ClaimedEvent) -> Result<()> {
        self.store
            .acknowledge(
                claimed.envelope.id,
                &claimed.consumer,
                claimed.attempt,
            )
            .await
    }

    /// Planifie une reprise ou place l'événement en quarantaine.
    pub async fn fail(
        &self,
        claimed: &ClaimedEvent,
        error: impl Into<String>,
    ) -> Result<Option<DeadLetter>> {
        let error = error.into();

        if claimed.attempt >= self.config.max_attempts {
            let dead_letter = self
                .store
                .move_to_dead_letter(
                    claimed.envelope.id,
                    &claimed.consumer,
                    claimed.attempt,
                    error,
                )
                .await?;

            return Ok(Some(dead_letter));
        }

        let exponent = claimed.attempt.saturating_sub(1).min(63);
        let multiplier = 1_u64.checked_shl(exponent).unwrap_or(u64::MAX);
        let delay_seconds = self
            .config
            .retry_base_seconds
            .saturating_mul(multiplier)
            .min(self.config.retry_max_seconds);

        self.store
            .schedule_retry(
                claimed.envelope.id,
                &claimed.consumer,
                claimed.attempt,
                Duration::from_secs(delay_seconds),
                error,
            )
            .await?;

        Ok(None)
    }

    /// Relit le journal sans modifier l'état de livraison.
    pub async fn replay_after(
        &self,
        sequence: u64,
        limit: usize,
    ) -> Result<Vec<SequencedEvent>> {
        self.store.replay_after(sequence, limit).await
    }

    pub async fn dead_letters(
        &self,
        consumer: Option<&str>,
        limit: usize,
    ) -> Result<Vec<DeadLetter>> {
        self.store.dead_letters(consumer, limit).await
    }
}

use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};

use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::envelope::EventEnvelope;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendReceipt {
    pub event_id: Uuid,
    pub sequence: u64,
    pub duplicate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencedEvent {
    pub sequence: u64,
    pub envelope: EventEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimedEvent {
    pub sequence: u64,
    pub envelope: EventEnvelope,
    pub consumer: String,
    pub attempt: u32,
    pub lease_until: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetter {
    pub sequence: u64,
    pub envelope: EventEnvelope,
    pub consumer: String,
    pub attempts: u32,
    pub last_error: String,
    pub dead_lettered_at: DateTime<Utc>,
}

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append(&self, envelope: EventEnvelope) -> Result<AppendReceipt>;

    async fn claim_batch(
        &self,
        consumer: &str,
        limit: usize,
        lease: Duration,
    ) -> Result<Vec<ClaimedEvent>>;

    async fn acknowledge(
        &self,
        event_id: Uuid,
        consumer: &str,
        attempt: u32,
    ) -> Result<()>;

    async fn schedule_retry(
        &self,
        event_id: Uuid,
        consumer: &str,
        attempt: u32,
        delay: Duration,
        error: String,
    ) -> Result<()>;

    async fn move_to_dead_letter(
        &self,
        event_id: Uuid,
        consumer: &str,
        attempt: u32,
        error: String,
    ) -> Result<DeadLetter>;

    async fn replay_after(
        &self,
        sequence: u64,
        limit: usize,
    ) -> Result<Vec<SequencedEvent>>;

    async fn dead_letters(
        &self,
        consumer: Option<&str>,
        limit: usize,
    ) -> Result<Vec<DeadLetter>>;
}

#[derive(Debug, Clone)]
struct StoredEvent {
    sequence: u64,
    envelope: EventEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeliveryStatus {
    InFlight,
    Delivered,
    DeadLetter,
}

#[derive(Debug, Clone)]
struct DeliveryState {
    status: DeliveryStatus,
    attempts: u32,
    available_at: DateTime<Utc>,
    lease_until: Option<DateTime<Utc>>,
    last_error: Option<String>,
}

impl DeliveryState {
    fn pending_now() -> Self {
        Self {
            status: DeliveryStatus::InFlight,
            attempts: 0,
            available_at: Utc::now(),
            lease_until: None,
            last_error: None,
        }
    }
}

#[derive(Default)]
struct StoreState {
    next_sequence: u64,
    events: HashMap<Uuid, StoredEvent>,
    order: Vec<Uuid>,
    idempotency: HashMap<String, Uuid>,
    deliveries: HashMap<(Uuid, String), DeliveryState>,
    dead_letters: Vec<DeadLetter>,
}

#[derive(Clone, Default)]
pub struct InMemoryEventStore {
    state: Arc<Mutex<StoreState>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_consumer(consumer: &str) -> Result<()> {
        if consumer.trim().is_empty() {
            bail!("consumer name cannot be empty");
        }
        Ok(())
    }

    fn validate_claim(
        delivery: &DeliveryState,
        attempt: u32,
    ) -> Result<()> {
        if delivery.status != DeliveryStatus::InFlight {
            bail!("event is not in flight");
        }

        if attempt == 0 || delivery.attempts != attempt {
            bail!("stale or invalid delivery attempt");
        }

        Ok(())
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append(&self, envelope: EventEnvelope) -> Result<AppendReceipt> {
        envelope.validate()?;

        let mut state = self.state.lock().await;

        if let Some(key) = envelope.idempotency_key.as_ref() {
            if let Some(existing_id) = state.idempotency.get(key) {
                let existing = state
                    .events
                    .get(existing_id)
                    .ok_or_else(|| anyhow::anyhow!(
                        "idempotency index references a missing event"
                    ))?;

                if existing.envelope.event_type != envelope.event_type
                    || existing.envelope.source != envelope.source
                    || existing.envelope.payload != envelope.payload
                {
                    bail!(
                        "idempotency key was reused with different event content"
                    );
                }

                return Ok(AppendReceipt {
                    event_id: existing.envelope.id,
                    sequence: existing.sequence,
                    duplicate: true,
                });
            }
        }

        if state.events.contains_key(&envelope.id) {
            bail!("event ID already exists: {}", envelope.id);
        }

        state.next_sequence = state
            .next_sequence
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("event sequence overflow"))?;

        let sequence = state.next_sequence;
        let event_id = envelope.id;

        if let Some(key) = envelope.idempotency_key.as_ref() {
            state.idempotency.insert(key.clone(), event_id);
        }

        state.order.push(event_id);
        state.events.insert(
            event_id,
            StoredEvent {
                sequence,
                envelope,
            },
        );

        Ok(AppendReceipt {
            event_id,
            sequence,
            duplicate: false,
        })
    }

    async fn claim_batch(
        &self,
        consumer: &str,
        limit: usize,
        lease: Duration,
    ) -> Result<Vec<ClaimedEvent>> {
        Self::validate_consumer(consumer)?;

        if limit == 0 {
            return Ok(Vec::new());
        }

        if lease.is_zero() {
            bail!("lease duration must be greater than zero");
        }

        let now = Utc::now();
        let lease_seconds = i64::try_from(lease.as_secs())
            .unwrap_or(i64::MAX)
            .max(1);
        let lease_until = now + chrono::Duration::seconds(lease_seconds);

        let mut state = self.state.lock().await;
        let ordered_ids = state.order.clone();
        let mut claimed = Vec::new();

        for event_id in ordered_ids {
            if claimed.len() >= limit {
                break;
            }

            let Some(stored) = state.events.get(&event_id).cloned() else {
                continue;
            };

            let key = (event_id, consumer.to_owned());
            let delivery = state
                .deliveries
                .entry(key)
                .or_insert_with(DeliveryState::pending_now);

            let eligible = match delivery.status {
                DeliveryStatus::Delivered | DeliveryStatus::DeadLetter => false,
                DeliveryStatus::InFlight => match delivery.lease_until {
                    Some(until) => until <= now,
                    None => delivery.available_at <= now,
                },
            };

            if !eligible || delivery.available_at > now {
                continue;
            }

            delivery.status = DeliveryStatus::InFlight;
            delivery.attempts = delivery
                .attempts
                .checked_add(1)
                .ok_or_else(|| anyhow::anyhow!("delivery attempt overflow"))?;
            delivery.lease_until = Some(lease_until);
            delivery.last_error = None;

            claimed.push(ClaimedEvent {
                sequence: stored.sequence,
                envelope: stored.envelope,
                consumer: consumer.to_owned(),
                attempt: delivery.attempts,
                lease_until,
            });
        }

        Ok(claimed)
    }

    async fn acknowledge(
        &self,
        event_id: Uuid,
        consumer: &str,
        attempt: u32,
    ) -> Result<()> {
        Self::validate_consumer(consumer)?;

        let mut state = self.state.lock().await;
        let key = (event_id, consumer.to_owned());
        let delivery = state
            .deliveries
            .get_mut(&key)
            .ok_or_else(|| anyhow::anyhow!("event was never claimed"))?;

        Self::validate_claim(delivery, attempt)?;

        delivery.status = DeliveryStatus::Delivered;
        delivery.lease_until = None;
        delivery.last_error = None;

        Ok(())
    }

    async fn schedule_retry(
        &self,
        event_id: Uuid,
        consumer: &str,
        attempt: u32,
        delay: Duration,
        error: String,
    ) -> Result<()> {
        Self::validate_consumer(consumer)?;

        let mut state = self.state.lock().await;
        let key = (event_id, consumer.to_owned());
        let delivery = state
            .deliveries
            .get_mut(&key)
            .ok_or_else(|| anyhow::anyhow!("event was never claimed"))?;

        Self::validate_claim(delivery, attempt)?;

        let delay_seconds = i64::try_from(delay.as_secs())
            .unwrap_or(i64::MAX);

        delivery.available_at =
            Utc::now() + chrono::Duration::seconds(delay_seconds);
        delivery.lease_until = None;
        delivery.last_error = Some(error);

        Ok(())
    }

    async fn move_to_dead_letter(
        &self,
        event_id: Uuid,
        consumer: &str,
        attempt: u32,
        error: String,
    ) -> Result<DeadLetter> {
        Self::validate_consumer(consumer)?;

        let mut state = self.state.lock().await;
        let key = (event_id, consumer.to_owned());

        let delivery = state
            .deliveries
            .get_mut(&key)
            .ok_or_else(|| anyhow::anyhow!("event was never claimed"))?;

        Self::validate_claim(delivery, attempt)?;
        delivery.status = DeliveryStatus::DeadLetter;
        delivery.lease_until = None;
        delivery.last_error = Some(error.clone());

        let attempts = delivery.attempts;
        let stored = state
            .events
            .get(&event_id)
            .ok_or_else(|| anyhow::anyhow!("event not found"))?;

        let dead_letter = DeadLetter {
            sequence: stored.sequence,
            envelope: stored.envelope.clone(),
            consumer: consumer.to_owned(),
            attempts,
            last_error: error,
            dead_lettered_at: Utc::now(),
        };

        state.dead_letters.push(dead_letter.clone());
        Ok(dead_letter)
    }

    async fn replay_after(
        &self,
        sequence: u64,
        limit: usize,
    ) -> Result<Vec<SequencedEvent>> {
        let state = self.state.lock().await;

        let mut events: Vec<SequencedEvent> = state
            .events
            .values()
            .filter(|event| event.sequence > sequence)
            .map(|event| SequencedEvent {
                sequence: event.sequence,
                envelope: event.envelope.clone(),
            })
            .collect();

        events.sort_by_key(|event| event.sequence);
        events.truncate(limit);
        Ok(events)
    }

    async fn dead_letters(
        &self,
        consumer: Option<&str>,
        limit: usize,
    ) -> Result<Vec<DeadLetter>> {
        let state = self.state.lock().await;

        let mut items: Vec<DeadLetter> = state
            .dead_letters
            .iter()
            .filter(|item| match consumer {
                Some(name) => item.consumer == name,
                None => true,
            })
            .cloned()
            .collect();

        items.sort_by_key(|item| item.dead_lettered_at);
        items.reverse();
        items.truncate(limit);
        Ok(items)
    }
}

use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::envelope::EventEnvelope;

#[derive(Debug, Clone)]
pub struct StoredEvent {
    pub envelope: EventEnvelope,
    pub stored_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct EventStore {
    events: Arc<RwLock<Vec<StoredEvent>>>,
}

impl EventStore {
    pub async fn append(&self, envelope: EventEnvelope) {
        self.events.write().await.push(StoredEvent {
            envelope,
            stored_at: Utc::now(),
        });
    }

    pub async fn get(&self, id: Uuid) -> Option<StoredEvent> {
        self.events
            .read()
            .await
            .iter()
            .find(|item| item.envelope.id() == id)
            .cloned()
    }

    pub async fn list(&self) -> Vec<StoredEvent> {
        self.events.read().await.clone()
    }

    pub async fn len(&self) -> usize {
        self.events.read().await.len()
    }

    pub async fn clear(&self) {
        self.events.write().await.clear();
    }
}

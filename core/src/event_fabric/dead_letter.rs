use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::envelope::EventEnvelope;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterEntry {
    pub envelope: EventEnvelope,
    pub reason: String,
    pub failed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct DeadLetterQueue {
    entries: Arc<RwLock<Vec<DeadLetterEntry>>>,
}

impl DeadLetterQueue {
    pub async fn push(
        &self,
        envelope: EventEnvelope,
        reason: impl Into<String>,
    ) {
        let entry = DeadLetterEntry {
            envelope,
            reason: reason.into(),
            failed_at: Utc::now(),
        };

        self.entries.write().await.push(entry);
    }

    pub async fn list(&self) -> Vec<DeadLetterEntry> {
        self.entries.read().await.clone()
    }

    pub async fn len(&self) -> usize {
        self.entries.read().await.len()
    }

    pub async fn clear(&self) {
        self.entries.write().await.clear();
    }
}

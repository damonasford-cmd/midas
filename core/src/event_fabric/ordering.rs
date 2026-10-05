use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventOrderingKey {
    pub stream: String,
}

impl EventOrderingKey {
    pub fn new(stream: impl Into<String>) -> Result<Self, String> {
        let stream = stream.into();

        if stream.trim().is_empty() {
            return Err("ordering stream cannot be empty".into());
        }

        Ok(Self { stream })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderingDecision {
    Accepted,
    OutOfOrder,
}

#[derive(Debug, Clone, Default)]
pub struct OrderingStore {
    sequences: Arc<RwLock<HashMap<String, u64>>>,
}

impl OrderingStore {
    pub async fn accept(
        &self,
        key: &EventOrderingKey,
        sequence: u64,
    ) -> OrderingDecision {
        let mut sequences = self.sequences.write().await;

        let expected = sequences
            .get(&key.stream)
            .copied()
            .unwrap_or(0);

        if sequence < expected {
            return OrderingDecision::OutOfOrder;
        }

        sequences.insert(key.stream.clone(), sequence.saturating_add(1));

        OrderingDecision::Accepted
    }

    pub async fn current(&self, stream: &str) -> Option<u64> {
        self.sequences.read().await.get(stream).copied()
    }
}

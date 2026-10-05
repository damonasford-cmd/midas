use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeduplicationDecision {
    New,
    Duplicate,
}

#[derive(Debug, Clone, Default)]
pub struct DeduplicationStore {
    seen: Arc<RwLock<HashSet<Uuid>>>,
}

impl DeduplicationStore {
    pub async fn check_and_record(
        &self,
        event_id: Uuid,
    ) -> DeduplicationDecision {
        let mut seen = self.seen.write().await;

        if seen.contains(&event_id) {
            DeduplicationDecision::Duplicate
        } else {
            seen.insert(event_id);
            DeduplicationDecision::New
        }
    }

    pub async fn contains(&self, event_id: Uuid) -> bool {
        self.seen.read().await.contains(&event_id)
    }

    pub async fn clear(&self) {
        self.seen.write().await.clear();
    }

    pub async fn len(&self) -> usize {
        self.seen.read().await.len()
    }
}

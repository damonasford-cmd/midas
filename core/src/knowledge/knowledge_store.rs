use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use super::{
    knowledge::Knowledge,
    knowledge_id::KnowledgeId,
};

#[derive(Debug, Clone)]
pub struct StoredKnowledge {
    pub knowledge: Knowledge,
}

#[derive(Debug, Clone, Default)]
pub struct KnowledgeStore {
    entries:
        Arc<RwLock<HashMap<KnowledgeId, StoredKnowledge>>>,
}

impl KnowledgeStore {
    pub async fn insert(
        &self,
        knowledge: Knowledge,
    ) -> Result<(), String> {
        knowledge.validate()?;

        self.entries.write().await.insert(
            knowledge.id,
            StoredKnowledge { knowledge },
        );

        Ok(())
    }

    pub async fn get(
        &self,
        id: KnowledgeId,
    ) -> Option<StoredKnowledge> {
        self.entries
            .read()
            .await
            .get(&id)
            .cloned()
    }

    pub async fn remove(
        &self,
        id: KnowledgeId,
    ) -> Option<StoredKnowledge> {
        self.entries.write().await.remove(&id)
    }

    pub async fn all(&self) -> Vec<StoredKnowledge> {
        self.entries
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }

    pub async fn len(&self) -> usize {
        self.entries.read().await.len()
    }

    pub async fn clear(&self) {
        self.entries.write().await.clear();
    }
}

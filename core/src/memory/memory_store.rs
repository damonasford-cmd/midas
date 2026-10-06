use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use super::{
    memory_id::MemoryId,
    memory_record::MemoryRecord,
};

#[derive(Debug, Clone)]
pub struct StoredMemory {
    pub record: MemoryRecord,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStore {
    records: Arc<RwLock<HashMap<MemoryId, StoredMemory>>>,
}

impl MemoryStore {
    pub async fn insert(
        &self,
        record: MemoryRecord,
    ) -> Result<(), String> {
        record.validate()?;

        self.records
            .write()
            .await
            .insert(
                record.id,
                StoredMemory { record },
            );

        Ok(())
    }

    pub async fn get(
        &self,
        id: MemoryId,
    ) -> Option<StoredMemory> {
        self.records
            .read()
            .await
            .get(&id)
            .cloned()
    }

    pub async fn remove(
        &self,
        id: MemoryId,
    ) -> Option<StoredMemory> {
        self.records.write().await.remove(&id)
    }

    pub async fn all(&self) -> Vec<StoredMemory> {
        self.records
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }

    pub async fn len(&self) -> usize {
        self.records.read().await.len()
    }

    pub async fn clear(&self) {
        self.records.write().await.clear();
    }
}

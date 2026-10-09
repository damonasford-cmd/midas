use anyhow::Result;
use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::{
    repository::MemoryRepository,
    types::{
        MemoryMatch, MemoryRecord, MemoryRelation, NewMemory,
        PendingMemoryEvent,
    },
};

pub struct MemoryService {
    repository: MemoryRepository,
}

impl MemoryService {
    pub fn new(repository: MemoryRepository) -> Self {
        Self { repository }
    }

    pub async fn initialize_check(&self) -> Result<()> {
        self.repository.health_check().await
    }

    pub async fn remember(&self, memory: NewMemory) -> Result<MemoryRecord> {
        memory.validate()?;

        let serialized = serde_json::to_vec(&memory)?;
        let content_hash = format!("{:x}", Sha256::digest(&serialized));

        self.repository
            .insert(Uuid::new_v4(), &content_hash, &memory)
            .await
    }

    pub async fn recall(&self, id: Uuid) -> Result<MemoryRecord> {
        self.repository.get(id).await
    }

    pub async fn recall_by_text(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<Vec<MemoryMatch>> {
        self.repository.recall_text(query, limit).await
    }

    pub async fn recall_by_meaning(
        &self,
        query_embedding: &[f32],
        embedding_model: &str,
        max_distance: f32,
        limit: i64,
    ) -> Result<Vec<MemoryMatch>> {
        self.repository
            .recall_semantic(
                query_embedding,
                embedding_model,
                max_distance,
                limit,
            )
            .await
    }

    pub async fn connect_memories(
        &self,
        from: Uuid,
        to: Uuid,
        relation_type: &str,
        metadata: Value,
    ) -> Result<MemoryRelation> {
        self.repository
            .relate(from, to, relation_type, metadata)
            .await
    }

    pub async fn archive(&self, id: Uuid, reason: &str) -> Result<()> {
        self.repository.archive(id, reason).await
    }

    pub async fn forget(&self, id: Uuid, reason: &str) -> Result<()> {
        self.repository.forget(id, reason).await
    }

    pub async fn pending_events(
        &self,
        limit: i64,
    ) -> Result<Vec<PendingMemoryEvent>> {
        self.repository.pending_events(limit).await
    }

    pub async fn mark_event_published(&self, id: Uuid) -> Result<()> {
        self.repository.mark_event_published(id).await
    }

    pub async fn mark_event_failed(
        &self,
        id: Uuid,
        error: &str,
    ) -> Result<()> {
        self.repository.mark_event_failed(id, error).await
    }
}

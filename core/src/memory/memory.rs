use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use super::{
    association::{
        MemoryAssociation,
        MemoryAssociationGraph,
    },
    forgetting::{
        ForgettingDecision,
        ForgettingPolicy,
    },
    health::{
        MemoryHealth,
        MemoryHealthState,
    },
    memory_id::MemoryId,
    memory_index::MemoryIndex,
    memory_record::MemoryRecord,
    memory_store::MemoryStore,
    retrieval::{
        MemoryQuery,
        MemoryQueryResult,
        MemoryRetrieval,
    },
    semantic_memory::SemanticMemory,
    working_memory::WorkingMemory,
};

#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub working_memory_capacity: usize,
    pub forgetting_policy: ForgettingPolicy,
    pub max_records_per_query: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            working_memory_capacity: 256,
            forgetting_policy: ForgettingPolicy::default(),
            max_records_per_query: 100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryOperationResult {
    Stored(MemoryId),
    AlreadyExists(MemoryId),
    Removed(MemoryId),
    NotFound,
}

pub struct Memory {
    config: MemoryConfig,

    store: MemoryStore,

    index: Arc<RwLock<MemoryIndex>>,

    associations:
        Arc<RwLock<MemoryAssociationGraph>>,

    working_memory:
        Arc<RwLock<WorkingMemory>>,

    semantic_memory:
        Arc<RwLock<SemanticMemory>>,

    retrieval: MemoryRetrieval,

    active: Arc<RwLock<bool>>,
}

impl Memory {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            working_memory: Arc::new(
                RwLock::new(
                    WorkingMemory::new(
                        config.working_memory_capacity,
                    ),
                ),
            ),

            config,

            store: MemoryStore::default(),

            index: Arc::new(
                RwLock::new(
                    MemoryIndex::default(),
                ),
            ),

            associations: Arc::new(
                RwLock::new(
                    MemoryAssociationGraph::default(),
                ),
            ),

            semantic_memory: Arc::new(
                RwLock::new(
                    SemanticMemory::default(),
                ),
            ),

            retrieval: MemoryRetrieval::default(),

            active: Arc::new(
                RwLock::new(false),
            ),
        }
    }

    pub async fn start(&self) {
        *self.active.write().await = true;
    }

    pub async fn stop(&self) {
        *self.active.write().await = false;
    }

    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }

    pub async fn store(
        &self,
        record: MemoryRecord,
    ) -> Result<MemoryOperationResult, String> {
        if !self.is_active().await {
            return Err(
                "memory subsystem is not active".into()
            );
        }

        if self.store.get(record.id).await.is_some() {
            return Ok(
                MemoryOperationResult::AlreadyExists(
                    record.id,
                )
            );
        }

        let id = record.id;

        if let super::content::MemoryContentKind::Text(
            text,
        ) = &record.content.kind
        {
            self.index
                .write()
                .await
                .index_text(id, text);
        }

        self.store.insert(record).await?;

        Ok(MemoryOperationResult::Stored(id))
    }

    pub async fn retrieve(
        &self,
        mut query: MemoryQuery,
    ) -> Result<Vec<MemoryQueryResult>, String> {
        if !self.is_active().await {
            return Err(
                "memory subsystem is not active".into()
            );
        }

        if query.limit == 0 {
            query.limit =
                self.config.max_records_per_query;
        }

        let records = self
            .store
            .all()
            .await
            .into_iter()
            .map(|stored| stored.record)
            .collect::<Vec<_>>();

        let filtered = self
            .retrieval
            .filter(
                records,
                &query.filter,
            );

        Ok(
            self.retrieval
                .rank(&filtered, &query),
        )
    }

    pub async fn get(
        &self,
        id: MemoryId,
    ) -> Option<MemoryRecord> {
        self.store
            .get(id)
            .await
            .map(|stored| stored.record)
    }

    pub async fn remove(
        &self,
        id: MemoryId,
    ) -> MemoryOperationResult {
        match self.store.remove(id).await {
            Some(_) => {
                MemoryOperationResult::Removed(id)
            }

            None => MemoryOperationResult::NotFound,
        }
    }

    pub async fn associate(
        &self,
        association: MemoryAssociation,
    ) -> Result<(), String> {
        self.associations
            .write()
            .await
            .add(association)
    }

    pub async fn associations(
        &self,
        id: MemoryId,
    ) -> Vec<MemoryAssociation> {
        self.associations
            .read()
            .await
            .related(id)
    }

    pub async fn add_to_working_memory(
        &self,
        id: MemoryId,
        priority: u8,
    ) -> Result<(), String> {
        if self.get(id).await.is_none() {
            return Err(
                "cannot add unknown memory to working memory"
                    .into(),
            );
        }

        self.working_memory
            .write()
            .await
            .push(
                super::working_memory::WorkingMemoryItem {
                    memory_id: id,
                    priority,
                },
            );

        Ok(())
    }

    pub async fn evaluate_forgetting(
        &self,
        id: MemoryId,
    ) -> Option<ForgettingDecision> {
        let record = self.get(id).await?;

        Some(
            self.config
                .forgetting_policy
                .evaluate(&record),
        )
    }

    pub async fn health(&self) -> MemoryHealth {
        let total = self.store.len().await;

        let active = if self.is_active().await {
            total
        } else {
            0
        };

        let state = if !self.is_active().await {
            MemoryHealthState::Unavailable
        } else {
            MemoryHealthState::Healthy
        };

        MemoryHealth {
            state,
            total_records: total,
            active_records: active,
            archived_records: 0,
            indexed_records: total,
            integrity_errors: 0,
            retrieval_available: true,
            persistence_available: true,
        }
    }

    pub fn store(&self) -> MemoryStore {
        self.store.clone()
    }

    pub fn working_memory(
        &self,
    ) -> Arc<RwLock<WorkingMemory>> {
        self.working_memory.clone()
    }

    pub fn semantic_memory(
        &self,
    ) -> Arc<RwLock<SemanticMemory>> {
        self.semantic_memory.clone()
    }

    pub fn index(
        &self,
    ) -> Arc<RwLock<MemoryIndex>> {
        self.index.clone()
    }

    pub fn correlation_id() -> Uuid {
        Uuid::new_v4()
    }
}

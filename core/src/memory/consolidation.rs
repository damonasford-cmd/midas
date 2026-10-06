use super::{
    association::MemoryAssociationGraph,
    memory_record::MemoryRecord,
    memory_store::MemoryStore,
};

#[derive(Debug, Clone)]
pub struct ConsolidationCandidate {
    pub memory: MemoryRecord,
    pub reason: String,
}

#[derive(Debug, Default)]
pub struct ConsolidationResult {
    pub processed: usize,
    pub consolidated: usize,
    pub preserved: usize,
}

#[derive(Debug, Default)]
pub struct ConsolidationEngine;

impl ConsolidationEngine {
    pub async fn analyze(
        &self,
        store: &MemoryStore,
        _associations: &MemoryAssociationGraph,
    ) -> ConsolidationResult {
        let memories = store.all().await;

        ConsolidationResult {
            processed: memories.len(),
            consolidated: 0,
            preserved: memories.len(),
        }
    }
}

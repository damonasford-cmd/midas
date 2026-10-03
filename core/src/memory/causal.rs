use super::unified::{MemoryKind, MemoryModality, UnifiedMemory};
use anyhow::Result;

#[derive(Clone)]
pub struct CausalMemory {
    memory: UnifiedMemory,
}

impl CausalMemory {
    pub fn new(memory: UnifiedMemory) -> Self {
        Self { memory }
    }

    pub fn record_relation(
        &self,
        source: impl Into<String>,
        relation: serde_json::Value,
    ) -> Result<()> {
        let entry = self.memory.create(
            MemoryKind::Causal,
            MemoryModality::Data,
            source,
            relation,
        );

        self.memory.remember(entry)
    }
}

use super::unified::{MemoryKind, MemoryModality, UnifiedMemory};
use anyhow::Result;

#[derive(Clone)]
pub struct SemanticMemory {
    memory: UnifiedMemory,
}

impl SemanticMemory {
    pub fn new(memory: UnifiedMemory) -> Self {
        Self { memory }
    }

    pub fn remember_fact(
        &self,
        source: impl Into<String>,
        fact: serde_json::Value,
    ) -> Result<()> {
        let entry = self.memory.create(
            MemoryKind::Semantic,
            MemoryModality::Data,
            source,
            fact,
        );

        self.memory.remember(entry)
    }
}

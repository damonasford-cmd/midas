use super::unified::{MemoryKind, MemoryModality, UnifiedMemory};
use anyhow::Result;

#[derive(Clone)]
pub struct ProceduralMemory {
    memory: UnifiedMemory,
}

impl ProceduralMemory {
    pub fn new(memory: UnifiedMemory) -> Self {
        Self { memory }
    }

    pub fn remember_procedure(
        &self,
        source: impl Into<String>,
        procedure: serde_json::Value,
    ) -> Result<()> {
        let entry = self.memory.create(
            MemoryKind::Procedural,
            MemoryModality::Code,
            source,
            procedure,
        );

        self.memory.remember(entry)
    }
}

use super::unified::{MemoryEntry, MemoryKind, MemoryModality, UnifiedMemory};
use anyhow::Result;

#[derive(Clone)]
pub struct EpisodicMemory {
    memory: UnifiedMemory,
}

impl EpisodicMemory {
    pub fn new(memory: UnifiedMemory) -> Self {
        Self { memory }
    }

    pub fn record(
        &self,
        source: impl Into<String>,
        event: serde_json::Value,
    ) -> Result<()> {
        let entry = self.memory.create(
            MemoryKind::Episodic,
            MemoryModality::Event,
            source,
            event,
        );

        self.memory.remember(entry)
    }
}

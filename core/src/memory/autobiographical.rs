use super::unified::{MemoryKind, MemoryModality, UnifiedMemory};
use anyhow::Result;

#[derive(Clone)]
pub struct AutobiographicalMemory {
    memory: UnifiedMemory,
}

impl AutobiographicalMemory {
    pub fn new(memory: UnifiedMemory) -> Self {
        Self { memory }
    }

    pub fn record(
        &self,
        event: serde_json::Value,
    ) -> Result<()> {
        let entry = self.memory.create(
            MemoryKind::Autobiographical,
            MemoryModality::Event,
            "midas.self",
            event,
        );

        self.memory.remember(entry)
    }
}

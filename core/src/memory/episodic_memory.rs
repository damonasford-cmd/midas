use chrono::{DateTime, Utc};

use super::{
    memory_id::MemoryId,
    memory_record::MemoryRecord,
};

#[derive(Debug, Clone)]
pub struct EpisodicMemoryEntry {
    pub memory_id: MemoryId,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct EpisodicMemory {
    entries: Vec<EpisodicMemoryEntry>,
}

impl EpisodicMemory {
    pub fn add(
        &mut self,
        record: &MemoryRecord,
    ) {
        self.entries.push(
            EpisodicMemoryEntry {
                memory_id: record.id,
                occurred_at: record
                    .temporal
                    .occurred_at
                    .unwrap_or(record.created_at),
            },
        );
    }

    pub fn all(&self) -> &[EpisodicMemoryEntry] {
        &self.entries
    }
}

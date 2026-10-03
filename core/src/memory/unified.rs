use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub kind: MemoryKind,
    pub modality: MemoryModality,
    pub source: String,
    pub content: Value,
    pub importance: f64,
    pub confidence: f64,
    pub tags: Vec<String>,
    pub related_memories: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryKind {
    Working,
    Episodic,
    Semantic,
    Procedural,
    Causal,
    Autobiographical,
    Decision,
    Learning,
    Error,
    Observation,
    Project,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryModality {
    Text,
    Image,
    Audio,
    Video,
    File,
    Code,
    Data,
    Sensor,
    Event,
    Multimodal,
    Other,
}

#[derive(Clone, Default)]
pub struct UnifiedMemory {
    entries: Arc<RwLock<Vec<MemoryEntry>>>,
}

impl UnifiedMemory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remember(&self, entry: MemoryEntry) -> Result<()> {
        let mut entries = self
            .entries
            .write()
            .map_err(|_| anyhow::anyhow!("Mémoire verrouillée"))?;

        entries.push(entry);

        Ok(())
    }

    pub fn create(
        &self,
        kind: MemoryKind,
        modality: MemoryModality,
        source: impl Into<String>,
        content: Value,
    ) -> MemoryEntry {
        MemoryEntry {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            kind,
            modality,
            source: source.into(),
            content,
            importance: 0.5,
            confidence: 1.0,
            tags: Vec::new(),
            related_memories: Vec::new(),
        }
    }

    pub fn all(&self) -> Result<Vec<MemoryEntry>> {
        let entries = self
            .entries
            .read()
            .map_err(|_| anyhow::anyhow!("Mémoire verrouillée"))?;

        Ok(entries.clone())
    }

    pub fn count(&self) -> Result<usize> {
        let entries = self
            .entries
            .read()
            .map_err(|_| anyhow::anyhow!("Mémoire verrouillée"))?;

        Ok(entries.len())
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryKind {
    Working,
    Episodic,
    Semantic,
    Procedural,
    Decision,
    Error,
    Learning,
    Multimodal,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub kind: MemoryKind,
    pub source: String,
    pub content: String,
    pub importance: f32,
    pub tags: Vec<String>,
}

impl Memory {
    pub fn new(
        kind: MemoryKind,
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            kind,
            source: source.into(),
            content: content.into(),
            importance: 1.0,
            tags: Vec::new(),
        }
    }

    pub fn with_importance(mut self, importance: f32) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    pub fn with_tags(
        mut self,
        tags: Vec<String>,
    ) -> Self {
        self.tags = tags;
        self
    }
}

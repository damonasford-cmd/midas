use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    pub title: Option<String>,
    pub tags: Vec<String>,

    pub language: Option<String>,

    pub importance: f32,
    pub confidence: f32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub custom: HashMap<String, String>,
}

impl Default for MemoryMetadata {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            title: None,
            tags: Vec::new(),
            language: None,
            importance: 0.5,
            confidence: 1.0,
            created_at: now,
            updated_at: now,
            custom: HashMap::new(),
        }
    }
}

impl MemoryMetadata {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.importance) {
            return Err("memory importance must be between 0 and 1".into());
        }

        if !(0.0..=1.0).contains(&self.confidence) {
            return Err("memory confidence must be between 0 and 1".into());
        }

        Ok(())
    }
}

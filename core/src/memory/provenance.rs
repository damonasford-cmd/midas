use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProvenance {
    pub source: String,
    pub captured_at: DateTime<Utc>,
    pub transformation_chain: Vec<String>,
    pub verification_sources: Vec<String>,
    pub confidence: f64,
}

impl MemoryProvenance {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            captured_at: Utc::now(),
            transformation_chain: Vec::new(),
            verification_sources: Vec::new(),
            confidence: 1.0,
        }
    }
}

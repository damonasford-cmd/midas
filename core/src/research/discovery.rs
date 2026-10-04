use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discovery {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub evidence: Vec<String>,
    pub implications: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Default)]
pub struct DiscoveryEngine;

impl DiscoveryEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
        confidence: f64,
    ) -> Discovery {
        Discovery {
            id: Uuid::new_v4(),
            title: title.into(),
            description: description.into(),
            evidence: Vec::new(),
            implications: Vec::new(),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureConcept {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub required_capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub uncertainties: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct FutureEngine;

impl FutureEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> FutureConcept {
        FutureConcept {
            id: Uuid::new_v4(),
            title: title.into(),
            description: description.into(),
            required_capabilities: Vec::new(),
            dependencies: Vec::new(),
            uncertainties: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

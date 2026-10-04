use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualScenario {
    pub id: Uuid,
    pub base_world: String,
    pub intervention: String,
    pub expected_consequences: Vec<String>,
    pub confidence: f64,
}

pub struct CounterfactualEngine;

impl Default for CounterfactualEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterfactualEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        base_world: impl Into<String>,
        intervention: impl Into<String>,
    ) -> CounterfactualScenario {
        CounterfactualScenario {
            id: Uuid::new_v4(),
            base_world: base_world.into(),
            intervention: intervention.into(),
            expected_consequences: Vec::new(),
            confidence: 0.0,
        }
    }
}

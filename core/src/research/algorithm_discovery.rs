use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmCandidate {
    pub id: Uuid,
    pub name: String,
    pub objective: String,
    pub description: String,
    pub complexity_estimate: Option<String>,
    pub expected_advantages: Vec<String>,
    pub known_limitations: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AlgorithmDiscoveryEngine;

impl AlgorithmDiscoveryEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn propose(
        &self,
        name: impl Into<String>,
        objective: impl Into<String>,
        description: impl Into<String>,
    ) -> AlgorithmCandidate {
        AlgorithmCandidate {
            id: Uuid::new_v4(),
            name: name.into(),
            objective: objective.into(),
            description: description.into(),
            complexity_estimate: None,
            expected_advantages: Vec::new(),
            known_limitations: Vec::new(),
        }
    }
}

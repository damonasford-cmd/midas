use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetacognitiveState {
    pub id: Uuid,
    pub confidence: f64,
    pub known_unknowns: Vec<String>,
    pub possible_unknowns: Vec<String>,
    pub assumptions: Vec<String>,
    pub detected_biases: Vec<String>,
    pub verification_required: bool,
}

pub struct MetacognitionEngine;

impl Default for MetacognitionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MetacognitionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(
        &self,
        confidence: f64,
        known_unknowns: Vec<String>,
        assumptions: Vec<String>,
    ) -> MetacognitiveState {
        MetacognitiveState {
            id: Uuid::new_v4(),
            confidence: confidence.clamp(0.0, 1.0),
            known_unknowns,
            possible_unknowns: Vec::new(),
            assumptions,
            detected_biases: Vec::new(),
            verification_required: confidence < 0.8,
        }
    }
}

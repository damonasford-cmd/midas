use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScientificHypothesis {
    pub id: Uuid,
    pub statement: String,
    pub domain: String,
    pub predictions: Vec<String>,
    pub evidence_for: Vec<String>,
    pub evidence_against: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ScienceEngine;

impl ScienceEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn hypothesis(
        &self,
        statement: impl Into<String>,
        domain: impl Into<String>,
    ) -> ScientificHypothesis {
        ScientificHypothesis {
            id: Uuid::new_v4(),
            statement: statement.into(),
            domain: domain.into(),
            predictions: Vec::new(),
            evidence_for: Vec::new(),
            evidence_against: Vec::new(),
        }
    }

    pub fn add_prediction(
        &self,
        hypothesis: &mut ScientificHypothesis,
        prediction: impl Into<String>,
    ) {
        hypothesis.predictions.push(prediction.into());
    }

    pub fn add_evidence_for(
        &self,
        hypothesis: &mut ScientificHypothesis,
        evidence: impl Into<String>,
    ) {
        hypothesis.evidence_for.push(evidence.into());
    }

    pub fn add_evidence_against(
        &self,
        hypothesis: &mut ScientificHypothesis,
        evidence: impl Into<String>,
    ) {
        hypothesis.evidence_against.push(evidence.into());
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::knowledge::KnowledgeId;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ReasoningMethod {
    Deductive,
    Inductive,
    Abductive,
    Analogical,
    Causal,
    Temporal,
    Statistical,
    Computational,
    Comparative,
    MultiMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningContext {
    pub id: Uuid,

    pub objective: String,

    pub relevant_knowledge:
        Vec<KnowledgeId>,

    pub assumptions: Vec<String>,

    pub constraints: Vec<String>,

    pub unknowns: Vec<String>,

    pub methods:
        Vec<ReasoningMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConclusion {
    pub statement: String,

    pub confidence: f32,

    pub supporting_knowledge:
        Vec<KnowledgeId>,

    pub assumptions: Vec<String>,

    pub unresolved_unknowns:
        Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub id: Uuid,

    pub method:
        ReasoningMethod,

    pub conclusion:
        ReasoningConclusion,

    pub reasoning_trace:
        Vec<String>,

    pub valid: bool,
}

impl ReasoningResult {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0)
            .contains(&self.conclusion.confidence)
        {
            return Err(
                "reasoning confidence must be between 0 and 1"
                    .into()
            );
        }

        if self.conclusion.statement.trim().is_empty() {
            return Err(
                "reasoning conclusion cannot be empty"
                    .into()
            );
        }

        Ok(())
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CausalRelationType {
    Causes,
    Enables,
    Prevents,
    Influences,
    CorrelatesWith,
    DependsOn,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalRelation {
    pub id: Uuid,
    pub cause: String,
    pub effect: String,
    pub relation: CausalRelationType,
    pub confidence: f64,
    pub evidence: Vec<Uuid>,
}

impl CausalRelation {
    pub fn new(
        cause: impl Into<String>,
        effect: impl Into<String>,
        relation: CausalRelationType,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            cause: cause.into(),
            effect: effect.into(),
            relation,
            confidence: confidence.clamp(0.0, 1.0),
            evidence: Vec::new(),
        }
    }

    pub fn add_evidence(&mut self, fact_id: Uuid) {
        self.evidence.push(fact_id);
    }
}

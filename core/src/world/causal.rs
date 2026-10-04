use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalRelation {
    pub id: Uuid,
    pub cause: String,
    pub effect: String,
    pub strength: f64,
    pub evidence: Vec<String>,
    pub confidence: f64,
}

#[derive(Default)]
pub struct CausalEngine {
    relations: Vec<CausalRelation>,
}

impl CausalEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        cause: impl Into<String>,
        effect: impl Into<String>,
        strength: f64,
        evidence: Vec<String>,
    ) -> CausalRelation {
        let relation = CausalRelation {
            id: Uuid::new_v4(),
            cause: cause.into(),
            effect: effect.into(),
            strength: strength.clamp(-1.0, 1.0),
            evidence,
            confidence: 0.5,
        };

        self.relations.push(relation.clone());

        relation
    }

    pub fn relations(&self) -> &[CausalRelation] {
        &self.relations
    }
}

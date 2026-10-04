use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: Uuid,
    pub title: String,
    pub problem: String,
    pub potential_solution: String,
    pub target: String,
    pub estimated_value: Option<f64>,
    pub confidence: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BusinessIntelligence;

impl BusinessIntelligence {
    pub fn new() -> Self {
        Self
    }

    pub fn opportunity(
        &self,
        title: impl Into<String>,
        problem: impl Into<String>,
        solution: impl Into<String>,
    ) -> Opportunity {
        Opportunity {
            id: Uuid::new_v4(),
            title: title.into(),
            problem: problem.into(),
            potential_solution: solution.into(),
            target: String::new(),
            estimated_value: None,
            confidence: 0.0,
            evidence: Vec::new(),
        }
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathematicalProblem {
    pub id: Uuid,
    pub statement: String,
    pub domain: String,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathematicalResult {
    pub problem_id: Uuid,
    pub solution: String,
    pub proof_or_justification: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Default)]
pub struct MathematicsEngine;

impl MathematicsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn problem(
        &self,
        statement: impl Into<String>,
        domain: impl Into<String>,
    ) -> MathematicalProblem {
        MathematicalProblem {
            id: Uuid::new_v4(),
            statement: statement.into(),
            domain: domain.into(),
            constraints: Vec::new(),
        }
    }

    pub fn result(
        &self,
        problem: &MathematicalProblem,
        solution: impl Into<String>,
        proof_or_justification: impl Into<String>,
        confidence: f64,
    ) -> MathematicalResult {
        MathematicalResult {
            problem_id: problem.id,
            solution: solution.into(),
            proof_or_justification: proof_or_justification.into(),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }
}

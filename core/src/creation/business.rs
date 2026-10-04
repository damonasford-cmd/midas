use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessConcept {
    pub id: Uuid,
    pub name: String,
    pub problem: String,
    pub solution: String,
    pub customer: String,
    pub revenue_model: String,
    pub costs: Vec<String>,
    pub risks: Vec<String>,
    pub status: BusinessStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessStatus {
    Idea,
    Validation,
    Pilot,
    Operating,
    Scaling,
    Paused,
    Closed,
}

#[derive(Debug, Clone, Default)]
pub struct BusinessEngine;

impl BusinessEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        name: impl Into<String>,
        problem: impl Into<String>,
        solution: impl Into<String>,
        customer: impl Into<String>,
    ) -> BusinessConcept {
        BusinessConcept {
            id: Uuid::new_v4(),
            name: name.into(),
            problem: problem.into(),
            solution: solution.into(),
            customer: customer.into(),
            revenue_model: String::new(),
            costs: Vec::new(),
            risks: Vec::new(),
            status: BusinessStatus::Idea,
        }
    }
}

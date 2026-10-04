use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringProject {
    pub id: Uuid,
    pub name: String,
    pub objective: String,
    pub specifications: Vec<String>,
    pub constraints: Vec<String>,
    pub safety_requirements: Vec<String>,
    pub status: EngineeringStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineeringStatus {
    Concept,
    Specification,
    Design,
    Prototype,
    Verification,
    Validation,
    Production,
    Deployed,
}

#[derive(Debug, Clone, Default)]
pub struct EngineeringEngine;

impl EngineeringEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        name: impl Into<String>,
        objective: impl Into<String>,
    ) -> EngineeringProject {
        EngineeringProject {
            id: Uuid::new_v4(),
            name: name.into(),
            objective: objective.into(),
            specifications: Vec::new(),
            constraints: Vec::new(),
            safety_requirements: Vec::new(),
            status: EngineeringStatus::Concept,
        }
    }
}

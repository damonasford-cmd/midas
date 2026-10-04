use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub customer_problem: String,
    pub value_proposition: String,
    pub status: ServiceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Concept,
    Validation,
    Pilot,
    Active,
    Scaling,
    Suspended,
    Retired,
}

#[derive(Debug, Clone, Default)]
pub struct ServiceEngine;

impl ServiceEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        customer_problem: impl Into<String>,
    ) -> Service {
        Service {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            customer_problem: customer_problem.into(),
            value_proposition: String::new(),
            status: ServiceStatus::Concept,
        }
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum SimulationStatus {
    NotStarted,
    Running,
    Completed,
    Failed,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum SimulationOutcome {
    Favorable,
    Unfavorable,
    Mixed,
    Inconclusive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub id: Uuid,

    pub objective: String,

    pub assumptions: Vec<String>,

    pub scenarios: Vec<String>,

    pub status: SimulationStatus,

    pub outcome:
        Option<SimulationOutcome>,

    pub findings: Vec<String>,

    pub confidence: f32,
}

impl Simulation {
    pub fn new(
        objective: impl Into<String>,
    ) -> Result<Self, String> {
        let objective = objective.into();

        if objective.trim().is_empty() {
            return Err(
                "simulation objective cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            objective,
            assumptions: Vec::new(),
            scenarios: Vec::new(),
            status: SimulationStatus::NotStarted,
            outcome: None,
            findings: Vec::new(),
            confidence: 0.0,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(
                "simulation confidence must be between 0 and 1"
                    .into()
            );
        }

        Ok(())
    }
}

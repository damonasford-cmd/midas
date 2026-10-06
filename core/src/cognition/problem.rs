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
pub enum ProblemState {
    Identified,
    Understood,
    Decomposed,
    Blocked,
    Solved,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemConstraint {
    pub id: Uuid,
    pub description: String,
    pub mandatory: bool,
}

impl ProblemConstraint {
    pub fn new(
        description: impl Into<String>,
        mandatory: bool,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "problem constraint cannot be empty".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            description,
            mandatory,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: Uuid,

    pub statement: String,

    pub state: ProblemState,

    pub complexity: f32,

    pub constraints: Vec<ProblemConstraint>,

    pub unknowns: Vec<String>,

    pub required_capabilities: Vec<Uuid>,

    pub success_conditions: Vec<String>,
}

impl Problem {
    pub fn new(
        statement: impl Into<String>,
    ) -> Result<Self, String> {
        let statement = statement.into();

        if statement.trim().is_empty() {
            return Err(
                "problem statement cannot be empty".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            statement,
            state: ProblemState::Identified,
            complexity: 0.5,
            constraints: Vec::new(),
            unknowns: Vec::new(),
            required_capabilities: Vec::new(),
            success_conditions: Vec::new(),
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.complexity) {
            return Err(
                "problem complexity must be between 0 and 1"
                    .into()
            );
        }

        if self.statement.trim().is_empty() {
            return Err(
                "problem statement cannot be empty".into()
            );
        }

        Ok(())
    }
}

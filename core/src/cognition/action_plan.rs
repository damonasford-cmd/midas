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
pub enum ActionPreparation {
    Planned,
    ReadyForAuthorization,
    Authorized,
    Blocked,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPlanStep {
    pub id: Uuid,

    pub sequence:
        u32,

    pub description:
        String,

    pub expected_result:
        String,

    pub rollback:
        Option<String>,

    pub requires_authorization:
        bool,

    pub risk:
        f32,
}

impl ActionPlanStep {
    pub fn new(
        sequence: u32,
        description: impl Into<String>,
        expected_result: impl Into<String>,
    ) -> Result<Self, String> {
        let description = description.into();
        let expected_result = expected_result.into();

        if description.trim().is_empty() {
            return Err(
                "action plan description cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            sequence,
            description,
            expected_result,
            rollback: None,
            requires_authorization: false,
            risk: 0.0,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.risk) {
            return Err(
                "action risk must be between 0 and 1"
                    .into()
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPlan {
    pub id: Uuid,

    pub objective:
        String,

    pub preparation:
        ActionPreparation,

    pub steps:
        Vec<ActionPlanStep>,

    pub expected_outcome:
        String,

    pub failure_strategy:
        Vec<String>,

    pub rollback_available:
        bool,
}

impl ActionPlan {
    pub fn new(
        objective: impl Into<String>,
        expected_outcome: impl Into<String>,
    ) -> Result<Self, String> {
        let objective = objective.into();
        let expected_outcome = expected_outcome.into();

        if objective.trim().is_empty() {
            return Err(
                "action plan objective cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            objective,
            preparation: ActionPreparation::Planned,
            steps: Vec::new(),
            expected_outcome,
            failure_strategy: Vec::new(),
            rollback_available: false,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        for step in &self.steps {
            step.validate()?;
        }

        Ok(())
    }
}

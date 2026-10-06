use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::foundation::{
    AuthorizationRequirement,
    Goal,
    RiskLevel,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum GoalPriority {
    Background,
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalContext {
    pub context_id: Uuid,

    pub goal: Goal,

    pub priority: GoalPriority,

    pub complexity: f32,

    pub expected_impact: f32,

    pub risk: RiskLevel,

    pub irreversibility: f32,

    pub authorization:
        AuthorizationRequirement,

    pub constraints: Vec<String>,

    pub resources_required: Vec<String>,

    pub success_conditions: Vec<String>,

    pub failure_conditions: Vec<String>,
}

impl GoalContext {
    pub fn new(goal: Goal) -> Self {
        Self {
            context_id: Uuid::new_v4(),

            goal,

            priority: GoalPriority::Normal,

            complexity: 0.5,

            expected_impact: 0.5,

            risk: RiskLevel::Low,

            irreversibility: 0.0,

            authorization:
                AuthorizationRequirement::None,

            constraints: Vec::new(),

            resources_required: Vec::new(),

            success_conditions: Vec::new(),

            failure_conditions: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        for value in [
            self.complexity,
            self.expected_impact,
            self.irreversibility,
        ] {
            if !(0.0..=1.0).contains(&value) {
                return Err(
                    "goal context scalar must be between 0 and 1"
                        .into(),
                );
            }
        }

        Ok(())
    }
}

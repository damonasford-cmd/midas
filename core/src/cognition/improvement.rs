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
pub enum ImprovementKind {
    Accuracy,
    Efficiency,
    Reliability,
    Safety,
    Quality,
    ResourceUsage,
    Reasoning,
    Planning,
    ExecutionPreparation,
    Knowledge,
    Capability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Improvement {
    pub id: Uuid,

    pub kind:
        ImprovementKind,

    pub target:
        String,

    pub current_state:
        String,

    pub desired_state:
        String,

    pub measurable:
        bool,

    pub expected_gain:
        f32,

    pub validated:
        bool,
}

impl Improvement {
    pub fn new(
        kind: ImprovementKind,
        target: impl Into<String>,
        current_state: impl Into<String>,
        desired_state: impl Into<String>,
    ) -> Result<Self, String> {
        let target = target.into();
        let current_state = current_state.into();
        let desired_state = desired_state.into();

        if target.trim().is_empty()
            || current_state.trim().is_empty()
            || desired_state.trim().is_empty()
        {
            return Err(
                "improvement fields cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            target,
            current_state,
            desired_state,
            measurable: true,
            expected_gain: 0.0,
            validated: false,
        })
    }
}

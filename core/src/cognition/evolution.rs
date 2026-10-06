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
pub enum EvolutionScope {
    Strategy,
    Architecture,
    Capability,
    Knowledge,
    Tooling,
    Infrastructure,
    ModelRuntime,
    Process,
    IdentityContinuity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub id: Uuid,

    pub scope:
        EvolutionScope,

    pub reason:
        String,

    pub current_state:
        String,

    pub proposed_state:
        String,

    pub expected_benefit:
        String,

    pub risks:
        Vec<String>,

    pub requires_validation:
        bool,

    pub requires_authorization:
        bool,

    pub rollback_plan:
        Option<String>,

    pub validated:
        bool,
}

impl EvolutionProposal {
    pub fn new(
        scope: EvolutionScope,
        reason: impl Into<String>,
        current_state: impl Into<String>,
        proposed_state: impl Into<String>,
    ) -> Result<Self, String> {
        let reason = reason.into();
        let current_state = current_state.into();
        let proposed_state = proposed_state.into();

        if reason.trim().is_empty()
            || current_state.trim().is_empty()
            || proposed_state.trim().is_empty()
        {
            return Err(
                "evolution proposal fields cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            scope,
            reason,
            current_state,
            proposed_state,
            expected_benefit: String::new(),
            risks: Vec::new(),
            requires_validation: true,
            requires_authorization: false,
            rollback_plan: None,
            validated: false,
        })
    }
}

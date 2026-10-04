use crate::foundation::contracts::{
    ActionImpact,
    ActionIntent,
    AuthorizationRequirement,
};

#[derive(Debug, Clone, Default)]
pub struct SecurityAuthorization;

impl SecurityAuthorization {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(
        &self,
        action: &ActionIntent,
    ) -> AuthorizationRequirement {
        match action.impact {
            ActionImpact::Negligible => AuthorizationRequirement::None,
            ActionImpact::Low => AuthorizationRequirement::SystemPolicy,
            ActionImpact::Moderate => AuthorizationRequirement::SystemPolicy,
            ActionImpact::High => AuthorizationRequirement::UserConfirmation,
            ActionImpact::Critical => AuthorizationRequirement::MultiFactor,
        }
    }
}

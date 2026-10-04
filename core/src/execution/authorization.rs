use crate::foundation::contracts::{
    ActionImpact,
    ActionIntent,
    AuthorizationRequirement,
};

#[derive(Debug, Clone, Default)]
pub struct AuthorizationEngine;

impl AuthorizationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn required_level(
        &self,
        action: &ActionIntent,
    ) -> AuthorizationRequirement {
        match action.impact {
            ActionImpact::Negligible | ActionImpact::Low => {
                AuthorizationRequirement::None
            }
            ActionImpact::Moderate => {
                AuthorizationRequirement::SystemPolicy
            }
            ActionImpact::High => {
                AuthorizationRequirement::UserConfirmation
            }
            ActionImpact::Critical => {
                AuthorizationRequirement::MultiFactor
            }
        }
    }

    pub fn requires_external_authorization(
        &self,
        action: &ActionIntent,
    ) -> bool {
        matches!(
            action.authorization,
            AuthorizationRequirement::ExternalAuthorization
        )
    }
}

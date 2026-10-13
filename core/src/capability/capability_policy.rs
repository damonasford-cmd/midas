use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

use super::{
    capability::Capability,
    capability_state::CapabilityState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAuthorization {
    pub capability_id: Uuid,
    pub principal_id: String,
    pub allowed: bool,
    pub missing_permissions: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct VerifiedPermissionSet {
    principal_id: String,
    permissions: BTreeSet<String>,
}

impl VerifiedPermissionSet {
    /// À appeler uniquement depuis la couche d'autorisation après
    /// vérification de l'identité et des droits auprès d'une source fiable.
    pub(crate) fn from_verified_grants(
        principal_id: String,
        permissions: impl IntoIterator<Item = String>,
    ) -> Result<Self> {
        if principal_id.trim().is_empty() {
            bail!("Verified principal cannot be empty");
        }

        let permissions: BTreeSet<String> = permissions.into_iter().collect();

        if permissions.iter().any(|p| p.trim().is_empty()) {
            bail!("Verified permissions cannot contain empty values");
        }

        Ok(Self {
            principal_id,
            permissions,
        })
    }

    pub fn principal_id(&self) -> &str {
        &self.principal_id
    }

    pub fn contains(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CapabilityPolicy;

impl CapabilityPolicy {
    pub fn authorize(
        &self,
        capability: &Capability,
        permissions: &VerifiedPermissionSet,
    ) -> CapabilityAuthorization {
        let missing_permissions: Vec<String> = capability
            .required_permissions
            .iter()
            .filter(|permission| !permissions.contains(permission))
            .cloned()
            .collect();

        let state_allows_action = capability.state.can_perform_actions();

        let allowed = state_allows_action && missing_permissions.is_empty();

        let reason = if !state_allows_action {
            format!(
                "Capability state '{}' does not permit actions",
                capability.state
            )
        } else if !missing_permissions.is_empty() {
            "One or more required permissions are missing".to_owned()
        } else {
            "Required permissions are present and capability state permits actions"
                .to_owned()
        };

        CapabilityAuthorization {
            capability_id: capability.id,
            principal_id: permissions.principal_id().to_owned(),
            allowed,
            missing_permissions,
            reason,
        }
    }

    pub fn validate_transition(
        current: CapabilityState,
        next: CapabilityState,
        latest_evaluation_is_current_and_successful: bool,
    ) -> Result<()> {
        if !current.can_transition_to(next) {
            bail!(
                "Forbidden capability state transition: '{}' -> '{}'",
                current,
                next
            );
        }

        if next == CapabilityState::Available
            && !latest_evaluation_is_current_and_successful
        {
            bail!(
                "A capability cannot become available without a current successful evaluation"
            );
        }

        Ok(())
    }
}

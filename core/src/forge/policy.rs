use anyhow::{bail, Result};
use async_trait::async_trait;

use crate::forge::types::{ForgeAction, ForgeRisk};

#[derive(Debug, Clone)]
pub struct ForgeAuthorization {
    pub allowed: bool,
    pub reason: String,
    pub authorization_id: Option<String>,
}

impl ForgeAuthorization {
    pub fn allow(reason: impl Into<String>) -> Self {
        Self {
            allowed: true,
            reason: reason.into(),
            authorization_id: None,
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            allowed: false,
            reason: reason.into(),
            authorization_id: None,
        }
    }
}

#[async_trait]
pub trait ForgeAuthorizationProvider: Send + Sync {
    async fn authorize(
        &self,
        action: &ForgeAction,
    ) -> Result<ForgeAuthorization>;
}

#[derive(Debug, Clone)]
pub struct ForgePolicy {
    pub require_isolation_for_code_execution: bool,
    pub require_explicit_authorization_for_high_risk: bool,
    pub require_explicit_authorization_for_critical_risk: bool,
    pub allow_unverified_external_effects: bool,
}

impl Default for ForgePolicy {
    fn default() -> Self {
        Self {
            require_isolation_for_code_execution: true,
            require_explicit_authorization_for_high_risk: true,
            require_explicit_authorization_for_critical_risk: true,
            allow_unverified_external_effects: false,
        }
    }
}

impl ForgePolicy {
    pub fn validate_authorization(
        &self,
        action: &ForgeAction,
        authorization: &ForgeAuthorization,
    ) -> Result<()> {
        if !authorization.allowed {
            bail!("Forge action denied: {}", authorization.reason);
        }

        if action.requires_human_authorization
            && authorization.authorization_id.is_none()
        {
            bail!("This action requires an explicit recorded authorization");
        }

        if action.risk == ForgeRisk::Critical
            && self.require_explicit_authorization_for_critical_risk
            && authorization.authorization_id.is_none()
        {
            bail!("Critical-risk action requires explicit authorization");
        }

        if action.risk == ForgeRisk::High
            && self.require_explicit_authorization_for_high_risk
            && authorization.authorization_id.is_none()
        {
            bail!("High-risk action requires explicit authorization");
        }

        Ok(())
    }

    pub fn validate_adapter_isolation(
        &self,
        action: &ForgeAction,
        adapter: &crate::forge::adapter::AdapterDescriptor,
    ) -> Result<()> {
        if self.require_isolation_for_code_execution
            && matches!(
                action.kind,
                crate::forge::types::ForgeActionKind::Build
                    | crate::forge::types::ForgeActionKind::Test
                    | crate::forge::types::ForgeActionKind::Repair
            )
            && !adapter.isolated
        {
            bail!("This operation requires an isolated execution adapter");
        }

        Ok(())
    }
}

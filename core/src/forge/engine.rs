use std::sync::Arc;

use anyhow::{bail, Result};

use crate::forge::{
    adapter::ForgeAdapterRegistry,
    policy::{ForgeAuthorizationProvider, ForgePolicy},
    types::{ForgeAction, ForgeJob, ForgeStepResult},
};

pub struct ForgeEngine {
    registry: Arc<ForgeAdapterRegistry>,
    authorization: Arc<dyn ForgeAuthorizationProvider>,
    policy: ForgePolicy,
}

impl ForgeEngine {
    pub fn new(
        registry: Arc<ForgeAdapterRegistry>,
        authorization: Arc<dyn ForgeAuthorizationProvider>,
        policy: ForgePolicy,
    ) -> Self {
        Self {
            registry,
            authorization,
            policy,
        }
    }

    pub async fn execute_action(
        &self,
        job: &ForgeJob,
        action: &ForgeAction,
    ) -> Result<ForgeStepResult> {
        action.validate()?;

        let authorization = self.authorization.authorize(action).await?;
        self.policy
            .validate_authorization(action, &authorization)?;

        let adapter = self.registry.find_for(action).await.ok_or_else(|| {
            anyhow::anyhow!(
                "No registered adapter currently supports operation '{}'. \
                 The missing capability must be researched, developed, acquired, \
                 or connected before this action can execute.",
                action.operation
            )
        })?;

        let descriptor = adapter.descriptor();
        self.policy
            .validate_adapter_isolation(action, &descriptor)?;

        let mut result = adapter.execute(job, action).await?;

        if !result.success {
            return Ok(result);
        }

        let verification = adapter.verify(job, action, &result).await?;
        result.evidence.extend(verification.evidence);

        if !verification.accepted {
            result.success = false;
            result.unresolved_issues.push(format!(
                "Independent verification did not accept the result: {}",
                verification.explanation
            ));
        }

        Ok(result)
    }
}

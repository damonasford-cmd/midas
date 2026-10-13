use anyhow::{Context, Result};
use chrono::{Duration, Utc};

use super::{
    capability::{Capability, CapabilityKey, NewCapability},
    capability_evaluation::{CapabilityCheck, CapabilityEvaluation},
    capability_policy::{
        CapabilityAccessRequest,
        CapabilityAccessResult,
        CapabilityPolicy,
    },
    capability_repository::CapabilityRepository,
    capability_state::{CapabilityState, CapabilityStateChange},
};

#[derive(Clone)]
pub struct CapabilityService {
    repository: CapabilityRepository,
}

impl CapabilityService {
    pub fn new(repository: CapabilityRepository) -> Self {
        Self { repository }
    }

    pub async fn register(&self, input: NewCapability) -> Result<Capability> {
        input.validate()?;
        self.repository.register(input).await
    }

    pub async fn get(&self, key: &str) -> Result<Option<Capability>> {
        let key = CapabilityKey::new(key.to_owned())?;
        self.repository.get_by_key(key.as_str()).await
    }

    pub async fn list(&self, limit: i64) -> Result<Vec<Capability>> {
        self.repository.list(limit).await
    }

    pub async fn add_dependency(
        &self,
        capability_key: &str,
        dependency_key: &str,
    ) -> Result<()> {
        let key = CapabilityKey::new(capability_key.to_owned())?;
        let dependency = CapabilityKey::new(dependency_key.to_owned())?;

        let capability = self.repository
            .get_by_key(key.as_str())
            .await?
            .context("capability not found")?;

        if capability.state == CapabilityState::Retired {
            anyhow::bail!("cannot modify a retired capability");
        }

        self.repository
            .add_dependency(capability.id, dependency.as_str())
            .await
    }

    pub async fn evaluate(
        &self,
        key: &str,
        evaluator: &str,
        checks: Vec<CapabilityCheck>,
        validity: Duration,
        summary: &str,
    ) -> Result<CapabilityEvaluation> {
        let capability = self.get(key).await?
            .context("capability not found")?;

        if capability.state == CapabilityState::Retired {
            anyhow::bail!("retired capabilities cannot be evaluated for use");
        }

        let evaluation = CapabilityEvaluation::new(
            capability.id,
            evaluator,
            checks,
            validity,
            summary,
        )?;

        self.repository.save_evaluation(&evaluation).await?;
        Ok(evaluation)
    }

    pub async fn set_state(
        &self,
        key: &str,
        next: CapabilityState,
        reason: &str,
        actor: &str,
    ) -> Result<Capability> {
        let capability = self.get(key).await?
            .context("capability not found")?;

        // The repository independently verifies that a successful,
        // unexpired evaluation exists before committing Available.
        let has_current_evaluation = next == CapabilityState::Available;

        let change = CapabilityStateChange::new(
            capability.state,
            next,
            reason,
            actor,
            has_current_evaluation,
        )?;

        self.repository
            .change_state(capability.id, &change)
            .await
    }

    pub async fn authorize_use(
        &self,
        request: &CapabilityAccessRequest,
    ) -> Result<CapabilityAccessResult> {
        let capability = self.get(&request.capability_key).await?
            .context("requested capability was not found")?;

        Ok(CapabilityPolicy::evaluate(&capability, request)?)
    }

    pub fn evaluation_is_fresh(
        evaluation: &CapabilityEvaluation,
        now: chrono::DateTime<Utc>,
    ) -> bool {
        evaluation.permits_available_state(now)
    }
}

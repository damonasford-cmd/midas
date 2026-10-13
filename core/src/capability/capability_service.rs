use anyhow::{bail, Result};
use uuid::Uuid;

use super::{
    capability::{Capability, NewCapability},
    capability_evaluation::CapabilityEvaluation,
    capability_repository::CapabilityRepository,
    capability_state::CapabilityState,
};

#[derive(Debug, Clone)]
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

    pub async fn get(&self, id: Uuid) -> Result<Option<Capability>> {
        self.repository.get(id).await
    }

    pub async fn record_evaluation(
        &self,
        evaluation: CapabilityEvaluation,
    ) -> Result<()> {
        if evaluation.completed_at > chrono::Utc::now() {
            bail!("Evaluation completion time cannot be in the future");
        }

        if evaluation.valid_until <= evaluation.completed_at {
            bail!("Evaluation must have a positive validity interval");
        }

        self.repository.save_evaluation(evaluation).await
    }

    pub async fn set_state(
        &self,
        id: Uuid,
        next: CapabilityState,
    ) -> Result<Capability> {
        self.repository.set_state(id, next).await
    }

    pub async fn add_dependency(
        &self,
        capability_id: Uuid,
        dependency_id: Uuid,
    ) -> Result<()> {
        self.repository
            .add_dependency(capability_id, dependency_id)
            .await
    }
}

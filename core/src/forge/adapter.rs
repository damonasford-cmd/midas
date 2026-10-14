use anyhow::Result;
use async_trait::async_trait;

use crate::forge::types::{
    ForgeAction, ForgeEvidence, ForgeJob, ForgeStepResult,
};

#[derive(Debug, Clone)]
pub struct AdapterDescriptor {
    pub id: String,
    pub name: String,
    pub domains: Vec<crate::forge::types::ForgeDomain>,
    pub operations: Vec<String>,
    pub supports_internal: bool,
    pub supports_external: bool,
    pub supports_physical_world: bool,
    pub isolated: bool,
    pub metadata: serde_json::Value,
}

#[async_trait]
pub trait ForgeAdapter: Send + Sync {
    fn descriptor(&self) -> AdapterDescriptor;

    fn supports(&self, action: &ForgeAction) -> bool;

    /// Performs the operation using the actual connected tool or system.
    async fn execute(
        &self,
        job: &ForgeJob,
        action: &ForgeAction,
    ) -> Result<ForgeStepResult>;

    /// Verifies outputs using evidence independent of the claimed summary
    /// wherever the connected environment permits it.
    async fn verify(
        &self,
        job: &ForgeJob,
        action: &ForgeAction,
        result: &ForgeStepResult,
    ) -> Result<VerificationResult>;
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub accepted: bool,
    pub evidence: Vec<ForgeEvidence>,
    pub explanation: String,
}

pub struct ForgeAdapterRegistry {
    adapters: tokio::sync::RwLock<
        Vec<std::sync::Arc<dyn ForgeAdapter>>,
    >,
}

impl ForgeAdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    pub async fn register(
        &self,
        adapter: std::sync::Arc<dyn ForgeAdapter>,
    ) -> Result<()> {
        let descriptor = adapter.descriptor();

        if descriptor.id.trim().is_empty() {
            anyhow::bail!("Forge adapter ID cannot be empty");
        }

        let mut adapters = self.adapters.write().await;

        if adapters
            .iter()
            .any(|item| item.descriptor().id == descriptor.id)
        {
            anyhow::bail!("Forge adapter already registered: {}", descriptor.id);
        }

        adapters.push(adapter);
        Ok(())
    }

    pub async fn find_for(
        &self,
        action: &ForgeAction,
    ) -> Option<std::sync::Arc<dyn ForgeAdapter>> {
        let adapters = self.adapters.read().await;

        adapters
            .iter()
            .find(|adapter| adapter.supports(action))
            .cloned()
    }

    pub async fn inventory(&self) -> Vec<AdapterDescriptor> {
        self.adapters
            .read()
            .await
            .iter()
            .map(|adapter| adapter.descriptor())
            .collect()
    }
}

impl Default for ForgeAdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

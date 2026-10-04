use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureDeployment {
    pub id: Uuid,
    pub target: String,
    pub version: String,
    pub environment: String,
    pub status: DeploymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Planned,
    Preparing,
    Deploying,
    Healthy,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, Default)]
pub struct InfrastructureDeploymentManager;

impl InfrastructureDeploymentManager {
    pub fn new() -> Self {
        Self
    }

    pub fn plan(
        &self,
        target: impl Into<String>,
        version: impl Into<String>,
        environment: impl Into<String>,
    ) -> InfrastructureDeployment {
        InfrastructureDeployment {
            id: Uuid::new_v4(),
            target: target.into(),
            version: version.into(),
            environment: environment.into(),
            status: DeploymentStatus::Planned,
        }
    }
}

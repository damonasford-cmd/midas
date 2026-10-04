use crate::capability::discovery::CapabilityCandidate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcquisitionMethod {
    Install,
    Build,
    Integrate,
    Configure,
    Connect,
    Create,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionPlan {
    pub capability: String,
    pub method: AcquisitionMethod,
    pub steps: Vec<String>,
    pub prerequisites: Vec<String>,
    pub risks: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CapabilityAcquisition;

impl CapabilityAcquisition {
    pub fn new() -> Self {
        Self
    }

    pub fn plan(&self, candidate: &CapabilityCandidate) -> AcquisitionPlan {
        let method = if candidate.source.starts_with("internal") {
            AcquisitionMethod::Build
        } else if candidate.source.starts_with("api") {
            AcquisitionMethod::Connect
        } else {
            AcquisitionMethod::Integrate
        };

        AcquisitionPlan {
            capability: candidate.name.clone(),
            method,
            steps: vec![
                "validate prerequisites".to_string(),
                "prepare isolated environment".to_string(),
                "acquire or build capability".to_string(),
                "run validation".to_string(),
                "register capability".to_string(),
            ],
            prerequisites: candidate.dependencies.clone(),
            risks: vec![
                "dependency incompatibility".to_string(),
                "security failure".to_string(),
                "resource exhaustion".to_string(),
            ],
        }
    }
}

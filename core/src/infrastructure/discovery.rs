use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfrastructureState {
    pub hostname: Option<String>,
    pub operating_system: Option<String>,
    pub architecture: Option<String>,
    pub cpu_count: Option<usize>,
    pub memory_bytes: Option<u64>,
    pub storage_bytes: Option<u64>,
    pub gpu_count: Option<u32>,
    pub network_available: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InfrastructureDiscovery;

impl InfrastructureDiscovery {
    pub fn new() -> Self {
        Self
    }

    pub fn empty_state(&self) -> InfrastructureState {
        InfrastructureState::default()
    }
}

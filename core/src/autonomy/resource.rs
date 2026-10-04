use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceState {
    pub cpu_available: Option<f64>,
    pub memory_available_bytes: Option<u64>,
    pub storage_available_bytes: Option<u64>,
    pub gpu_count: Option<u32>,
    pub network_available: bool,
    pub external_services_available: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ResourceEngine;

impl ResourceEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn can_allocate(
        &self,
        resources: &ResourceState,
        cpu_required: f64,
        memory_required: u64,
    ) -> bool {
        let cpu_ok = resources
            .cpu_available
            .map(|value| value >= cpu_required)
            .unwrap_or(false);

        let memory_ok = resources
            .memory_available_bytes
            .map(|value| value >= memory_required)
            .unwrap_or(false);

        cpu_ok && memory_ok
    }
}

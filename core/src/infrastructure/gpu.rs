use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub index: u32,
    pub name: String,
    pub vendor: String,
    pub memory_bytes: Option<u64>,
    pub driver_version: Option<String>,
    pub compute_available: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GpuManager {
    devices: Vec<GpuDevice>,
}

impl GpuManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, device: GpuDevice) {
        self.devices.push(device);
    }

    pub fn devices(&self) -> &[GpuDevice] {
        &self.devices
    }

    pub fn available(&self) -> Vec<&GpuDevice> {
        self.devices
            .iter()
            .filter(|device| device.compute_available)
            .collect()
    }
}

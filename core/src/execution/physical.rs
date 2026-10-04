use crate::foundation::contracts::ActionIntent;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalDevice {
    pub id: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub interface: PhysicalInterface,
    pub online: bool,
    pub safety_state: SafetyState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicalInterface {
    Usb,
    Serial,
    Ethernet,
    Wifi,
    Bluetooth,
    Http,
    WebSocket,
    Mqtt,
    OpcUa,
    ManufacturerApi,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyState {
    Unknown,
    Safe,
    Restricted,
    EmergencyStop,
}

pub trait PhysicalDriver: Send + Sync {
    fn device(&self) -> &PhysicalDevice;

    fn execute(
        &self,
        action: &ActionIntent,
    ) -> Result<String>;
}

#[derive(Default)]
pub struct PhysicalRegistry {
    devices: Vec<Box<dyn PhysicalDriver>>,
}

impl PhysicalRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        device: Box<dyn PhysicalDriver>,
    ) {
        self.devices.push(device);
    }

    pub fn online_devices(&self) -> Vec<String> {
        self.devices
            .iter()
            .filter(|device| device.device().online)
            .map(|device| device.device().name.clone())
            .collect()
    }
}

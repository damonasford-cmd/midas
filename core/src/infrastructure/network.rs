use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub connected: bool,
    pub interface_count: usize,
    pub latency_ms: Option<u128>,
    pub bandwidth_mbps: Option<f64>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            connected: false,
            interface_count: 0,
            latency_ms: None,
            bandwidth_mbps: None,
        }
    }
}

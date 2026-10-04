use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResiliencePolicy {
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub enable_failover: bool,
    pub enable_degraded_mode: bool,
}

impl Default for ResiliencePolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_ms: 1_000,
            enable_failover: true,
            enable_degraded_mode: true,
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub provider: String,
    pub healthy: bool,
    pub latency_ms: Option<u128>,
    pub available_models: usize,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeHealthChecker;

impl RuntimeHealthChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn result(
        &self,
        provider: impl Into<String>,
        healthy: bool,
        latency_ms: Option<u128>,
        available_models: usize,
    ) -> RuntimeHealth {
        RuntimeHealth {
            provider: provider.into(),
            healthy,
            latency_ms,
            available_models,
            checked_at: Utc::now(),
        }
    }
}

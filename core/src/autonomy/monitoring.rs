use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSignal {
    pub id: Uuid,
    pub component: String,
    pub healthy: bool,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct AutonomyMonitor;

impl AutonomyMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn signal(
        &self,
        component: impl Into<String>,
        healthy: bool,
        message: impl Into<String>,
    ) -> HealthSignal {
        HealthSignal {
            id: Uuid::new_v4(),
            component: component.into(),
            healthy,
            message: message.into(),
            timestamp: Utc::now(),
        }
    }
}

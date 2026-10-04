use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub component: String,
    pub healthy: bool,
    pub message: String,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct HealthRegistry {
    components: Vec<ComponentHealth>,
}

impl HealthRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(
        &mut self,
        component: impl Into<String>,
        healthy: bool,
        message: impl Into<String>,
    ) {
        let component = component.into();

        if let Some(existing) = self
            .components
            .iter_mut()
            .find(|item| item.component == component)
        {
            existing.healthy = healthy;
            existing.message = message.into();
            existing.checked_at = Utc::now();
            return;
        }

        self.components.push(ComponentHealth {
            component,
            healthy,
            message: message.into(),
            checked_at: Utc::now(),
        });
    }

    pub fn all(&self) -> &[ComponentHealth] {
        &self.components
    }

    pub fn overall_healthy(&self) -> bool {
        self.components.iter().all(|item| item.healthy)
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingCapability {
    pub id: Uuid,
    pub requested_for: String,
    pub description: String,
    pub importance: f64,
    pub discovered_at: DateTime<Utc>,
    pub status: MissingCapabilityStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissingCapabilityStatus {
    Detected,
    Searching,
    CandidateFound,
    Building,
    Testing,
    Integrated,
    Blocked,
}

#[derive(Debug, Clone, Default)]
pub struct MissingCapabilityRegistry {
    items: Vec<MissingCapability>,
}

impl MissingCapabilityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        requested_for: impl Into<String>,
        description: impl Into<String>,
        importance: f64,
    ) -> Uuid {
        let id = Uuid::new_v4();

        self.items.push(MissingCapability {
            id,
            requested_for: requested_for.into(),
            description: description.into(),
            importance: importance.clamp(0.0, 1.0),
            discovered_at: Utc::now(),
            status: MissingCapabilityStatus::Detected,
        });

        id
    }

    pub fn all(&self) -> &[MissingCapability] {
        &self.items
    }

    pub fn update_status(
        &mut self,
        id: Uuid,
        status: MissingCapabilityStatus,
    ) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.status = status;
            true
        } else {
            false
        }
    }
}

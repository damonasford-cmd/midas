use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub priority: NotificationPriority,
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct NotificationEngine;

impl NotificationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        title: impl Into<String>,
        message: impl Into<String>,
        priority: NotificationPriority,
    ) -> Notification {
        Notification {
            id: Uuid::new_v4(),
            title: title.into(),
            message: message.into(),
            priority,
            channels: Vec::new(),
        }
    }
}

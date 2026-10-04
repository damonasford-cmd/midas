use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub action: String,
    pub target: String,
    pub success: bool,
    pub details: String,
}

#[derive(Debug, Clone, Default)]
pub struct AuditLog {
    records: Vec<AuditRecord>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(
        &mut self,
        actor: impl Into<String>,
        action: impl Into<String>,
        target: impl Into<String>,
        success: bool,
        details: impl Into<String>,
    ) -> Uuid {
        let record = AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            actor: actor.into(),
            action: action.into(),
            target: target.into(),
            success,
            details: details.into(),
        };

        let id = record.id;
        self.records.push(record);
        id
    }

    pub fn all(&self) -> &[AuditRecord] {
        &self.records
    }
}

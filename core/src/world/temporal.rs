
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalStatus {
    Scheduled,
    Occurred,
    Ongoing,
    Cancelled,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalRecord {
    pub id: Uuid,
    pub subject_id: Option<Uuid>,
    pub event_type: String,
    pub description: String,
    pub status: TemporalStatus,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub recorded_at: DateTime<Utc>,
    pub source_provenance_id: Option<Uuid>,
}

impl TemporalRecord {
    pub fn new(
        event_type: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let event_type = event_type.into();
        let description = description.into();

        if event_type.trim().is_empty() || description.trim().is_empty() {
            return Err("temporal record fields cannot be empty".into());
        }

        Ok(Self {
            id: Uuid::new_v4(),
            subject_id: None,
            event_type,
            description,
            status: TemporalStatus::Unknown,
            starts_at: None,
            ends_at: None,
            recorded_at: Utc::now(),
            source_provenance_id: None,
        })
    }

    pub fn set_interval(
        &mut self,
        starts_at: Option<DateTime<Utc>>,
        ends_at: Option<DateTime<Utc>>,
    ) -> Result<(), String> {
        if let (Some(start), Some(end)) = (starts_at, ends_at) {
            if end < start {
                return Err("temporal interval ends before it starts".into());
            }
        }

        self.starts_at = starts_at;
        self.ends_at = ends_at;

        Ok(())
    }

    pub fn is_active_at(&self, time: DateTime<Utc>) -> bool {
        if self.status == TemporalStatus::Cancelled {
            return false;
        }

        self.starts_at.map_or(true, |start| time >= start)
            && self.ends_at.map_or(true, |end| time <= end)
    }
}

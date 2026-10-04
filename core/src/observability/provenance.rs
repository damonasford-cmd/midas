use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: Uuid,
    pub subject: String,
    pub source: String,
    pub captured_at: DateTime<Utc>,
    pub transformations: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ProvenanceRegistry {
    records: Vec<ProvenanceRecord>,
}

impl ProvenanceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(
        &mut self,
        subject: impl Into<String>,
        source: impl Into<String>,
        confidence: f64,
    ) -> Uuid {
        let record = ProvenanceRecord {
            id: Uuid::new_v4(),
            subject: subject.into(),
            source: source.into(),
            captured_at: Utc::now(),
            transformations: Vec::new(),
            confidence: confidence.clamp(0.0, 1.0),
        };

        let id = record.id;
        self.records.push(record);
        id
    }
}

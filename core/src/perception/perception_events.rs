use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::observation::Observation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionEvent {
    pub id: Uuid,
    pub event_type: String,
    pub source: String,
    pub occurred_at: DateTime<Utc>,
    pub payload: serde_json::Value,
}

impl PerceptionEvent {
    pub fn observation_ingested(observation: &Observation) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: "perception.observation_ingested".to_owned(),
            source: "midas.perception".to_owned(),
            occurred_at: observation.acquired_at,
            payload: json!({
                "observation_id": observation.id,
                "source_id": observation.source_id,
                "modality": observation.modality,
                "content_type": observation.content_type,
                "source_uri": observation.source_uri,
                "source_record_id": observation.source_record_id,
                "occurred_at": observation.occurred_at,
                "acquired_at": observation.acquired_at,
                "status": observation.status,
            }),
        }
    }
}

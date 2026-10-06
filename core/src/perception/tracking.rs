use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum TrackingStatus {
    Tentative,
    Confirmed,
    Lost,
    Reacquired,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedEntity {
    pub id: Uuid,

    pub detection_ids:
        Vec<Uuid>,

    pub label:
        String,

    pub status:
        TrackingStatus,

    pub first_seen:
        chrono::DateTime<chrono::Utc>,

    pub last_seen:
        chrono::DateTime<chrono::Utc>,

    pub observation_count:
        u64,

    pub confidence:
        f32,

    pub velocity:
        Option<[f64; 3]>,

    pub position:
        Option<[f64; 3]>,
}

impl TrackedEntity {
    pub fn new(
        label: impl Into<String>,
        detection_id: Uuid,
    ) -> Self {
        let now = chrono::Utc::now();

        Self {
            id: Uuid::new_v4(),
            detection_ids:
                vec![detection_id],
            label: label.into(),
            status:
                TrackingStatus::Tentative,
            first_seen: now,
            last_seen: now,
            observation_count: 1,
            confidence: 0.5,
            velocity: None,
            position: None,
        }
    }

    pub fn update(
        &mut self,
        detection_id: Uuid,
        confidence: f32,
    ) {
        self.detection_ids
            .push(detection_id);

        self.last_seen =
            chrono::Utc::now();

        self.observation_count += 1;

        self.confidence =
            confidence.clamp(0.0, 1.0);

        if self.observation_count >= 2 {
            self.status =
                TrackingStatus::Confirmed;
        }
    }
}

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
pub enum PerceivedEventKind {
    Appearance,
    Disappearance,
    Movement,
    StateChange,
    ThresholdCrossed,
    Failure,
    Recovery,
    Communication,
    EnvironmentalChange,
    HumanInteraction,
    SecurityEvent,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceivedEvent {
    pub id:
        Uuid,

    pub kind:
        PerceivedEventKind,

    pub description:
        String,

    pub observation_ids:
        Vec<Uuid>,

    pub confidence:
        f32,

    pub occurred_at:
        chrono::DateTime<chrono::Utc>,
}

impl PerceivedEvent {
    pub fn new(
        kind: PerceivedEventKind,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "perceived event description cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            description,
            observation_ids: Vec::new(),
            confidence: 0.5,
            occurred_at:
                chrono::Utc::now(),
        })
    }
}

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
pub enum DetectionClass {
    Person,
    Animal,
    Vehicle,
    Device,
    Machine,
    Building,
    Object,
    Document,
    Software,
    Event,
    Signal,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DetectionConfidence {
    pub classification: f32,
    pub localization: f32,
    pub temporal: f32,
}

impl DetectionConfidence {
    pub fn overall(&self) -> f32 {
        (
            self.classification
                + self.localization
                + self.temporal
        ) / 3.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub id: Uuid,

    pub class:
        DetectionClass,

    pub label:
        String,

    pub confidence:
        DetectionConfidence,

    pub observation_id:
        Uuid,

    pub attributes:
        Vec<String>,
}

impl Detection {
    pub fn validate(&self) -> Result<(), String> {
        for value in [
            self.confidence.classification,
            self.confidence.localization,
            self.confidence.temporal,
        ] {
            if !(0.0..=1.0).contains(&value) {
                return Err(
                    "detection confidence must be between 0 and 1"
                        .into(),
                );
            }
        }

        Ok(())
    }
}

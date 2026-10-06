use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum UncertaintyKind {
    Measurement,
    Sensor,
    Source,
    Classification,
    Localization,
    Temporal,
    Environmental,
    Occlusion,
    Inference,
    Fusion,
    Prediction,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uncertainty {
    pub kind: UncertaintyKind,

    pub value: f32,

    pub explanation: Option<String>,

    pub reducible: bool,

    pub known_source: bool,
}

impl Uncertainty {
    pub fn new(
        kind: UncertaintyKind,
        value: f32,
    ) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&value) {
            return Err(
                "uncertainty must be between 0 and 1"
                    .into(),
            );
        }

        Ok(Self {
            kind,
            value,
            explanation: None,
            reducible: true,
            known_source: false,
        })
    }

    pub fn confidence(&self) -> f32 {
        1.0 - self.value
    }
}

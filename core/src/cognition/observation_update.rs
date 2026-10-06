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
pub enum ObservationUpdateKind {
    NewInformation,
    ExpectedOutcome,
    UnexpectedOutcome,
    Failure,
    PartialSuccess,
    EnvironmentalChange,
    ExternalEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationUpdate {
    pub id: Uuid,

    pub kind:
        ObservationUpdateKind,

    pub description:
        String,

    pub source:
        Option<String>,

    pub expected:
        bool,

    pub measured_value:
        Option<f64>,

    pub unit:
        Option<String>,

    pub timestamp:
        chrono::DateTime<chrono::Utc>,
}

impl ObservationUpdate {
    pub fn new(
        kind: ObservationUpdateKind,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "observation update cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            description,
            source: None,
            expected: false,
            measured_value: None,
            unit: None,
            timestamp: chrono::Utc::now(),
        })
    }
}

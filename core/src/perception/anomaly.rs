use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum AnomalySeverity {
    Informational,
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum AnomalyType {
    SensorFailure,
    DataCorruption,
    UnexpectedChange,
    Outlier,
    Contradiction,
    Security,
    Environmental,
    Operational,
    Communication,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub id: Uuid,

    pub kind:
        AnomalyType,

    pub severity:
        AnomalySeverity,

    pub description:
        String,

    pub detected_at:
        chrono::DateTime<chrono::Utc>,

    pub source_id:
        Option<Uuid>,

    pub confidence:
        f32,

    pub requires_investigation:
        bool,

    pub resolved:
        bool,
}

impl Anomaly {
    pub fn new(
        kind: AnomalyType,
        severity: AnomalySeverity,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "anomaly description cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            severity,
            description,
            detected_at: chrono::Utc::now(),
            source_id: None,
            confidence: 0.5,
            requires_investigation:
                matches!(
                    severity,
                    AnomalySeverity::Major
                        | AnomalySeverity::Critical
                ),
            resolved: false,
        })
    }
}

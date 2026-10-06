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
pub enum EnvironmentCondition {
    Normal,
    LowLight,
    HighNoise,
    Obstructed,
    UnstableNetwork,
    ExtremeWeather,
    SensorDegraded,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentContext {
    pub id:
        Uuid,

    pub conditions:
        Vec<EnvironmentCondition>,

    pub description:
        Option<String>,

    pub confidence:
        f32,

    pub observed_at:
        chrono::DateTime<chrono::Utc>,
}

impl EnvironmentContext {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            conditions:
                vec![
                    EnvironmentCondition::Unknown
                ],
            description: None,
            confidence: 0.0,
            observed_at:
                chrono::Utc::now(),
        }
    }
}

impl Default for EnvironmentContext {
    fn default() -> Self {
        Self::new()
    }
}

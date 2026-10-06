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
pub enum LocalizationMethod {
    GNSS,
    GPS,
    Inertial,
    Visual,
    Network,
    SensorFusion,
    MapMatching,
    Manual,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationEstimate {
    pub id: Uuid,

    pub method:
        LocalizationMethod,

    pub latitude:
        Option<f64>,

    pub longitude:
        Option<f64>,

    pub altitude:
        Option<f64>,

    pub accuracy_meters:
        Option<f64>,

    pub confidence:
        f32,

    pub observed_at:
        chrono::DateTime<chrono::Utc>,
}

impl LocalizationEstimate {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(
                "localization confidence must be between 0 and 1"
                    .into(),
            );
        }

        if let Some(latitude) = self.latitude {
            if !(-90.0..=90.0)
                .contains(&latitude)
            {
                return Err(
                    "latitude is invalid".into()
                );
            }
        }

        if let Some(longitude) = self.longitude {
            if !(-180.0..=180.0)
                .contains(&longitude)
            {
                return Err(
                    "longitude is invalid".into()
                );
            }
        }

        Ok(())
    }
}

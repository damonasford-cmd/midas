use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    modality::PerceptionModality,
    provenance::PerceptionProvenance,
    quality::DataQuality,
    uncertainty::Uncertainty,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ObservationStatus {
    Raw,
    Processed,
    Interpreted,
    Fused,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptualObservation {
    pub id: Uuid,

    pub modality:
        PerceptionModality,

    pub content:
        String,

    pub status:
        ObservationStatus,

    pub quality:
        DataQuality,

    pub uncertainty:
        Vec<Uncertainty>,

    pub provenance:
        Vec<PerceptionProvenance>,

    pub observed_at:
        chrono::DateTime<chrono::Utc>,

    pub valid_until:
        Option<chrono::DateTime<chrono::Utc>>,

    pub tags:
        Vec<String>,
}

impl PerceptualObservation {
    pub fn new(
        modality: PerceptionModality,
        content: impl Into<String>,
    ) -> Result<Self, String> {
        let content = content.into();

        if content.trim().is_empty() {
            return Err(
                "perceptual observation cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            modality,
            content,
            status: ObservationStatus::Raw,
            quality: DataQuality::default(),
            uncertainty: Vec::new(),
            provenance: Vec::new(),
            observed_at: chrono::Utc::now(),
            valid_until: None,
            tags: Vec::new(),
        })
    }

    pub fn confidence(&self) -> f32 {
        if self.uncertainty.is_empty() {
            self.quality.overall
        } else {
            let uncertainty =
                self.uncertainty
                    .iter()
                    .map(|item| item.value)
                    .sum::<f32>()
                    / self.uncertainty.len() as f32;

            self.quality.overall
                * (1.0 - uncertainty)
        }
    }
}

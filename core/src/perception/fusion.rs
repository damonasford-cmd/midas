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
pub enum FusionMethod {
    None,
    Weighted,
    ReliabilityWeighted,
    Temporal,
    Spatial,
    Probabilistic,
    Evidential,
    Consensus,
    CrossModal,
    MultiStage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionSource {
    pub observation_id:
        Uuid,

    pub reliability:
        f32,

    pub weight:
        f32,

    pub accepted:
        bool,

    pub reason:
        Option<String>,
}

impl FusionSource {
    pub fn effective_weight(&self) -> f32 {
        self.weight
            .clamp(0.0, 1.0)
            * self.reliability
                .clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionResult {
    pub id:
        Uuid,

    pub method:
        FusionMethod,

    pub source_observations:
        Vec<FusionSource>,

    pub fused_statement:
        String,

    pub confidence:
        f32,

    pub residual_uncertainty:
        f32,

    pub conflicts:
        Vec<String>,

    pub degraded_sources:
        Vec<Uuid>,
}

impl FusionResult {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0)
            .contains(&self.confidence)
        {
            return Err(
                "fusion confidence must be between 0 and 1"
                    .into(),
            );
        }

        if !(0.0..=1.0)
            .contains(&self.residual_uncertainty)
        {
            return Err(
                "fusion uncertainty must be between 0 and 1"
                    .into(),
            );
        }

        Ok(())
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCandidate {
    pub provider: String,
    pub model: String,
    pub available: bool,
    pub health_score: f64,
    pub performance_score: f64,
    pub cost_score: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ModelSelector;

impl ModelSelector {
    pub fn new() -> Self {
        Self
    }

    pub fn score(candidate: &ModelCandidate) -> f64 {
        candidate.health_score * 0.4
            + candidate.performance_score * 0.4
            + candidate.cost_score * 0.2
    }

    pub fn select(
        &self,
        candidates: &[ModelCandidate],
    ) -> Option<ModelCandidate> {
        candidates
            .iter()
            .filter(|candidate| candidate.available)
            .max_by(|a, b| {
                Self::score(a)
                    .total_cmp(&Self::score(b))
            })
            .cloned()
    }
}

use serde::{Deserialize, Serialize};

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
pub enum CognitiveDepth {
    Minimal,
    Standard,
    Deep,
    VeryDeep,
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveDepthPolicy {
    pub default_depth: CognitiveDepth,
    pub complexity_threshold: f32,
    pub risk_threshold: f32,
    pub impact_threshold: f32,
    pub irreversibility_threshold: f32,
}

impl Default for CognitiveDepthPolicy {
    fn default() -> Self {
        Self {
            default_depth: CognitiveDepth::Standard,
            complexity_threshold: 0.6,
            risk_threshold: 0.6,
            impact_threshold: 0.6,
            irreversibility_threshold: 0.5,
        }
    }
}

impl CognitiveDepthPolicy {
    pub fn select_depth(
        &self,
        complexity: f32,
        risk: f32,
        impact: f32,
        irreversibility: f32,
    ) -> CognitiveDepth {
        let complexity = complexity.clamp(0.0, 1.0);
        let risk = risk.clamp(0.0, 1.0);
        let impact = impact.clamp(0.0, 1.0);
        let irreversibility =
            irreversibility.clamp(0.0, 1.0);

        if risk >= 0.9
            || impact >= 0.9
            || irreversibility >= 0.9
        {
            CognitiveDepth::Maximum
        } else if risk >= self.risk_threshold
            || impact >= self.impact_threshold
            || irreversibility
                >= self.irreversibility_threshold
        {
            CognitiveDepth::VeryDeep
        } else if complexity >= 0.85 {
            CognitiveDepth::Deep
        } else if complexity
            >= self.complexity_threshold
        {
            CognitiveDepth::Standard
        } else {
            CognitiveDepth::Minimal
        }
    }
}

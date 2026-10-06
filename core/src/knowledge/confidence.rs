use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
)]
pub struct ConfidenceScore {
    pub value: f32,
    pub evidence_weight: f32,
    pub source_weight: f32,
    pub temporal_weight: f32,
    pub consistency_weight: f32,
}

impl Default for ConfidenceScore {
    fn default() -> Self {
        Self {
            value: 0.0,
            evidence_weight: 0.0,
            source_weight: 0.0,
            temporal_weight: 0.0,
            consistency_weight: 0.0,
        }
    }
}

impl ConfidenceScore {
    pub fn new(value: f32) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&value) {
            return Err(
                "confidence must be between 0 and 1".into()
            );
        }

        Ok(Self {
            value,
            ..Self::default()
        })
    }

    pub fn clamp(&mut self) {
        self.value = self.value.clamp(0.0, 1.0);
        self.evidence_weight =
            self.evidence_weight.clamp(0.0, 1.0);
        self.source_weight =
            self.source_weight.clamp(0.0, 1.0);
        self.temporal_weight =
            self.temporal_weight.clamp(0.0, 1.0);
        self.consistency_weight =
            self.consistency_weight.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConfidenceUpdate {
    pub previous: f32,
    pub new_value: f32,
    pub delta: f32,
}

impl ConfidenceUpdate {
    pub fn from_values(
        previous: f32,
        new_value: f32,
    ) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&previous)
            || !(0.0..=1.0).contains(&new_value)
        {
            return Err(
                "confidence values must be between 0 and 1"
                    .into(),
            );
        }

        Ok(Self {
            previous,
            new_value,
            delta: new_value - previous,
        })
    }
}

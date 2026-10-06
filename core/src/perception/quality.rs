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
pub enum QualityLevel {
    Unknown,
    VeryLow,
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DataQuality {
    pub overall: f32,

    pub completeness: f32,

    pub integrity: f32,

    pub freshness: f32,

    pub resolution: f32,

    pub consistency: f32,

    pub level: QualityLevel,
}

impl Default for DataQuality {
    fn default() -> Self {
        Self {
            overall: 0.0,
            completeness: 0.0,
            integrity: 0.0,
            freshness: 0.0,
            resolution: 0.0,
            consistency: 0.0,
            level: QualityLevel::Unknown,
        }
    }
}

impl DataQuality {
    pub fn calculate(
        completeness: f32,
        integrity: f32,
        freshness: f32,
        resolution: f32,
        consistency: f32,
    ) -> Self {
        let values = [
            completeness,
            integrity,
            freshness,
            resolution,
            consistency,
        ];

        let overall =
            values.iter().sum::<f32>()
                / values.len() as f32;

        let overall =
            overall.clamp(0.0, 1.0);

        let level =
            if overall >= 0.9 {
                QualityLevel::VeryHigh
            } else if overall >= 0.75 {
                QualityLevel::High
            } else if overall >= 0.5 {
                QualityLevel::Moderate
            } else if overall >= 0.25 {
                QualityLevel::Low
            } else if overall > 0.0 {
                QualityLevel::VeryLow
            } else {
                QualityLevel::Unknown
            };

        Self {
            overall,
            completeness:
                completeness.clamp(0.0, 1.0),
            integrity:
                integrity.clamp(0.0, 1.0),
            freshness:
                freshness.clamp(0.0, 1.0),
            resolution:
                resolution.clamp(0.0, 1.0),
            consistency:
                consistency.clamp(0.0, 1.0),
            level,
        }
    }
}

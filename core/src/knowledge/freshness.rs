use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum FreshnessStatus {
    Fresh,
    Aging,
    Stale,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessPolicy {
    pub fresh_after: Duration,
    pub stale_after: Duration,
}

impl Default for FreshnessPolicy {
    fn default() -> Self {
        Self {
            fresh_after: Duration::hours(24),
            stale_after: Duration::days(30),
        }
    }
}

impl FreshnessPolicy {
    pub fn evaluate(
        &self,
        timestamp: DateTime<Utc>,
    ) -> FreshnessStatus {
        let age = Utc::now() - timestamp;

        if age <= self.fresh_after {
            FreshnessStatus::Fresh
        } else if age <= self.stale_after {
            FreshnessStatus::Aging
        } else {
            FreshnessStatus::Stale
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalInterval {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
}

impl TemporalInterval {
    pub fn open(start: DateTime<Utc>) -> Self {
        Self {
            start,
            end: None,
        }
    }

    pub fn closed(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Self, String> {
        if end < start {
            return Err(
                "La fin d'un intervalle ne peut pas précéder son début."
                    .to_string(),
            );
        }

        Ok(Self {
            start,
            end: Some(end),
        })
    }

    pub fn contains(&self, timestamp: DateTime<Utc>) -> bool {
        if timestamp < self.start {
            return false;
        }

        match self.end {
            Some(end) => timestamp <= end,
            None => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalState {
    pub observed_at: DateTime<Utc>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
}

impl TemporalState {
    pub fn now() -> Self {
        Self {
            observed_at: Utc::now(),
            valid_from: None,
            valid_until: None,
        }
    }
}

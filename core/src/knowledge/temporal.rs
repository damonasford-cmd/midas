use chrono::{DateTime, Utc};
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
pub enum TemporalValidity {
    Unknown,
    Current,
    Historical,
    Future,
    Expired,
    Timeless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTemporalScope {
    pub validity: TemporalValidity,

    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,

    pub observed_at: Option<DateTime<Utc>>,
}

impl Default for KnowledgeTemporalScope {
    fn default() -> Self {
        Self {
            validity: TemporalValidity::Unknown,
            valid_from: None,
            valid_until: None,
            observed_at: Some(Utc::now()),
        }
    }
}

impl KnowledgeTemporalScope {
    pub fn is_valid_now(&self) -> bool {
        let now = Utc::now();

        if let Some(from) = self.valid_from {
            if now < from {
                return false;
            }
        }

        if let Some(until) = self.valid_until {
            if now > until {
                return false;
            }
        }

        true
    }
}

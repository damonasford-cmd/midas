use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTemporalPosition {
    pub occurred_at: Option<DateTime<Utc>>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
}

impl Default for MemoryTemporalPosition {
    fn default() -> Self {
        Self {
            occurred_at: Some(Utc::now()),
            valid_from: None,
            valid_until: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MemoryTimeline {
    entries: Vec<MemoryTemporalPosition>,
}

impl MemoryTimeline {
    pub fn add(&mut self, position: MemoryTemporalPosition) {
        self.entries.push(position);
    }

    pub fn all(&self) -> &[MemoryTemporalPosition] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

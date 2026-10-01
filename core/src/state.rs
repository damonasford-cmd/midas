use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreState {
    pub started_at: DateTime<Utc>,
    pub cycle: u64,
    pub running: bool,
}

impl Default for CoreState {
    fn default() -> Self {
        Self {
            started_at: Utc::now(),
            cycle: 0,
            running: false,
        }
    }
}

impl CoreState {
    pub fn next_cycle(&mut self) {
        self.cycle += 1;
    }
}

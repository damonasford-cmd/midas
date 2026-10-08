use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MidasClock {
    started_at: DateTime<Utc>,
    monotonic_start: Instant,
}

impl MidasClock {
    pub fn new() -> Self {
        Self {
            started_at: Utc::now(),
            monotonic_start: Instant::now(),
        }
    }

    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub fn uptime(&self) -> Duration {
        self.monotonic_start.elapsed()
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.uptime().as_secs()
    }
}

impl Default for MidasClock {
    fn default() -> Self {
        Self::new()
    }
}

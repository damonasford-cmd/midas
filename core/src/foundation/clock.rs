use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, Default)]
pub struct MidasClock;

impl MidasClock {
    pub fn new() -> Self {
        Self
    }

    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    pub fn unix_timestamp(&self) -> i64 {
        self.now().timestamp()
    }
}

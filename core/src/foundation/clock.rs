use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct MidasClock;

impl MidasClock {
    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

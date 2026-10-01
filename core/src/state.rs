use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreState {
    pub status: CoreStatus,
    pub cycle: u64,
    pub started_at: Option<DateTime<Utc>>,
    pub last_cycle_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub total_cycles: u64,
    pub successful_cycles: u64,
    pub failed_cycles: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreStatus {
    Initializing,
    Running,
    Degraded,
    Maintenance,
    Stopping,
    Stopped,
}

impl Default for CoreState {
    fn default() -> Self {
        Self {
            status: CoreStatus::Initializing,
            cycle: 0,
            started_at: None,
            last_cycle_at: None,
            last_error: None,
            total_cycles: 0,
            successful_cycles: 0,
            failed_cycles: 0,
        }
    }
}

impl CoreState {
    pub fn set_running(&mut self) {
        self.status = CoreStatus::Running;

        if self.started_at.is_none() {
            self.started_at = Some(Utc::now());
        }
    }

    pub fn set_degraded(&mut self) {
        self.status = CoreStatus::Degraded;
    }

    pub fn set_maintenance(&mut self) {
        self.status = CoreStatus::Maintenance;
    }

    pub fn set_stopping(&mut self) {
        self.status = CoreStatus::Stopping;
    }

    pub fn set_stopped(&mut self) {
        self.status = CoreStatus::Stopped;
    }

    pub fn next_cycle(&mut self) {
        self.cycle = self.cycle.saturating_add(1);
        self.total_cycles =
            self.total_cycles.saturating_add(1);
        self.last_cycle_at = Some(Utc::now());
    }

    pub fn record_success(&mut self) {
        self.successful_cycles =
            self.successful_cycles.saturating_add(1);

        self.last_error = None;
    }

    pub fn record_failure(
        &mut self,
        error: impl Into<String>,
    ) {
        self.failed_cycles =
            self.failed_cycles.saturating_add(1);

        self.last_error = Some(error.into());
        self.status = CoreStatus::Degraded;
    }

    pub fn is_running(&self) -> bool {
        self.status == CoreStatus::Running
    }

    pub fn is_stopped(&self) -> bool {
        self.status == CoreStatus::Stopped
    }

    pub fn uptime_seconds(&self) -> Option<i64> {
        self.started_at.map(|started| {
            (Utc::now() - started).num_seconds()
        })
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_cycles == 0 {
            return 0.0;
        }

        self.successful_cycles as f64
            / self.total_cycles as f64
    }

    pub fn summary(&self) -> String {
        format!(
            "status={:?}, cycle={}, total={}, \
             succès={}, échecs={}, taux={:.2}%",
            self.status,
            self.cycle,
            self.total_cycles,
            self.successful_cycles,
            self.failed_cycles,
            self.success_rate() * 100.0,
        )
    }
}

use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

#[derive(Debug, Default)]
pub struct EventFabricMetrics {
    published: AtomicU64,
    dispatched: AtomicU64,
    duplicates: AtomicU64,
    failures: AtomicU64,
    dead_letters: AtomicU64,
}

impl EventFabricMetrics {
    pub fn record_published(&self) {
        self.published.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_dispatched(&self) {
        self.dispatched.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_duplicate(&self) {
        self.duplicates.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failures.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_dead_letter(&self) {
        self.dead_letters.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> EventMetricSnapshot {
        EventMetricSnapshot {
            published: self.published.load(Ordering::Relaxed),
            dispatched: self.dispatched.load(Ordering::Relaxed),
            duplicates: self.duplicates.load(Ordering::Relaxed),
            failures: self.failures.load(Ordering::Relaxed),
            dead_letters: self.dead_letters.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EventMetricSnapshot {
    pub published: u64,
    pub dispatched: u64,
    pub duplicates: u64,
    pub failures: u64,
    pub dead_letters: u64,
}

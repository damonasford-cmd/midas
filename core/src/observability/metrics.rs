use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct MetricsRegistry {
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(
        &mut self,
        name: impl Into<String>,
    ) {
        *self.counters.entry(name.into()).or_insert(0) += 1;
    }

    pub fn counter(
        &self,
        name: &str,
    ) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }

    pub fn set_gauge(
        &mut self,
        name: impl Into<String>,
        value: f64,
    ) {
        self.gauges.insert(name.into(), value);
    }

    pub fn gauge(
        &self,
        name: &str,
    ) -> Option<f64> {
        self.gauges.get(name).copied()
    }
}

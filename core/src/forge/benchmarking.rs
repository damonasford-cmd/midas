use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub id: Uuid,
    pub name: String,
    pub started_at: DateTime<Utc>,
    pub duration_ms: u128,
    pub iterations: u64,
    pub success_count: u64,
    pub failure_count: u64,
}

#[derive(Debug, Clone, Default)]
pub struct BenchmarkEngine;

impl BenchmarkEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn measure<F>(
        &self,
        name: impl Into<String>,
        iterations: u64,
        mut operation: F,
    ) -> BenchmarkResult
    where
        F: FnMut() -> bool,
    {
        let started_at = Utc::now();
        let timer = Instant::now();

        let mut success_count = 0;
        let mut failure_count = 0;

        for _ in 0..iterations {
            if operation() {
                success_count += 1;
            } else {
                failure_count += 1;
            }
        }

        BenchmarkResult {
            id: Uuid::new_v4(),
            name: name.into(),
            started_at,
            duration_ms: timer.elapsed().as_millis(),
            iterations,
            success_count,
            failure_count,
        }
    }
}

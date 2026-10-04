use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageState {
    pub total_bytes: u64,
    pub available_bytes: u64,
}

impl StorageState {
    pub fn utilization(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }

        1.0 - (self.available_bytes as f64 / self.total_bytes as f64)
    }
}

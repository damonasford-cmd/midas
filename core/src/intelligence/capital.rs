use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapitalState {
    pub cash: f64,
    pub invested: f64,
    pub liabilities: f64,
    pub reserved: f64,
    pub currency: String,
}

impl CapitalState {
    pub fn net_value(&self) -> f64 {
        self.cash + self.invested - self.liabilities
    }

    pub fn available_capital(&self) -> f64 {
        (self.cash - self.reserved).max(0.0)
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialPosition {
    pub id: Uuid,
    pub account: String,
    pub currency: String,
    pub balance: f64,
    pub available: f64,
    pub exposure: f64,
}

#[derive(Debug, Clone, Default)]
pub struct FinancialEngine;

impl FinancialEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn position(
        &self,
        account: impl Into<String>,
        currency: impl Into<String>,
        balance: f64,
        available: f64,
        exposure: f64,
    ) -> FinancialPosition {
        FinancialPosition {
            id: Uuid::new_v4(),
            account: account.into(),
            currency: currency.into(),
            balance,
            available,
            exposure,
        }
    }
}

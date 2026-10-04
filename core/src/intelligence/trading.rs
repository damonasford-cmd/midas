use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketType {
    Forex,
    Futures,
    Index,
    Commodity,
    Bond,
    Equity,
    Crypto,
    Derivative,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketObservation {
    pub id: Uuid,
    pub market: MarketType,
    pub symbol: String,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub volume: Option<f64>,
    pub timestamp: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradingAction {
    Observe,
    Buy,
    Sell,
    Close,
    Cancel,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingIntent {
    pub id: Uuid,
    pub symbol: String,
    pub action: TradingAction,
    pub quantity: f64,
    pub rationale: String,
    pub maximum_loss: Option<f64>,
    pub requires_execution_authorization: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TradingEngine;

impl TradingEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn observation(
        &self,
        market: MarketType,
        symbol: impl Into<String>,
    ) -> MarketObservation {
        MarketObservation {
            id: Uuid::new_v4(),
            market,
            symbol: symbol.into(),
            bid: None,
            ask: None,
            last: None,
            volume: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            source: String::new(),
        }
    }

    pub fn intent(
        &self,
        symbol: impl Into<String>,
        action: TradingAction,
        quantity: f64,
        rationale: impl Into<String>,
    ) -> TradingIntent {
        TradingIntent {
            id: Uuid::new_v4(),
            symbol: symbol.into(),
            action,
            quantity,
            rationale: rationale.into(),
            maximum_loss: None,
            requires_execution_authorization: true,
        }
    }
}

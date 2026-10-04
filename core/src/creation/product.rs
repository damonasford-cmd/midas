use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub target_market: String,
    pub requirements: Vec<String>,
    pub status: ProductStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductStatus {
    Concept,
    Research,
    Design,
    Prototype,
    Testing,
    Production,
    Market,
    Retired,
}

#[derive(Debug, Clone, Default)]
pub struct ProductEngine;

impl ProductEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        target_market: impl Into<String>,
    ) -> Product {
        Product {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            target_market: target_market.into(),
            requirements: Vec::new(),
            status: ProductStatus::Concept,
        }
    }
}

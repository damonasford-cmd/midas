use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub idempotency_key: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Prepared,
    Committed,
    RolledBack,
    Failed,
}

#[derive(Debug, Clone, Default)]
pub struct TransactionManager {
    processed_keys: Arc<Mutex<HashSet<String>>>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_duplicate(
        &self,
        idempotency_key: &str,
    ) -> bool {
        self.processed_keys
            .lock()
            .map(|keys| keys.contains(idempotency_key))
            .unwrap_or(false)
    }

    pub fn register(
        &self,
        idempotency_key: impl Into<String>,
    ) -> bool {
        let key = idempotency_key.into();

        match self.processed_keys.lock() {
            Ok(mut keys) => keys.insert(key),
            Err(_) => false,
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::{FoundationError, FoundationResult};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Created,
    Prepared,
    Committed,
    RolledBack,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub idempotency_key: String,
    pub operation: String,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct TransactionManager;

impl TransactionManager {
    pub fn new() -> Self {
        Self
    }

    pub fn begin(
        &self,
        operation: impl Into<String>,
        idempotency_key: impl Into<String>,
    ) -> Transaction {
        let now = Utc::now();

        Transaction {
            id: Uuid::new_v4(),
            idempotency_key: idempotency_key.into(),
            operation: operation.into(),
            status: TransactionStatus::Created,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn prepare(
        &self,
        transaction: &mut Transaction,
    ) -> FoundationResult<()> {
        if transaction.status != TransactionStatus::Created {
            return Err(FoundationError::Transaction(
                "transaction cannot be prepared from current state"
                    .to_string(),
            ));
        }

        transaction.status = TransactionStatus::Prepared;
        transaction.updated_at = Utc::now();

        Ok(())
    }

    pub fn commit(
        &self,
        transaction: &mut Transaction,
    ) -> FoundationResult<()> {
        if transaction.status != TransactionStatus::Prepared {
            return Err(FoundationError::Transaction(
                "only prepared transactions can be committed"
                    .to_string(),
            ));
        }

        transaction.status = TransactionStatus::Committed;
        transaction.updated_at = Utc::now();

        Ok(())
    }

    pub fn rollback(
        &self,
        transaction: &mut Transaction,
    ) -> FoundationResult<()> {
        match transaction.status {
            TransactionStatus::Committed => {
                return Err(FoundationError::Transaction(
                    "committed transaction cannot be rolled back by this manager"
                        .to_string(),
                ));
            }

            TransactionStatus::RolledBack => return Ok(()),

            _ => {}
        }

        transaction.status = TransactionStatus::RolledBack;
        transaction.updated_at = Utc::now();

        Ok(())
    }
}

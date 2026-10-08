use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::errors::{
    FoundationError,
    FoundationResult,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum TransactionStatus {
    Created,
    Prepared,
    Committed,
    RolledBack,
    Failed,
}

#[derive(Debug, Clone)]
pub enum TransactionOperation {
    Record {
        key: String,
        value: serde_json::Value,
    },
    Remove {
        key: String,
    },
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub status: TransactionStatus,
    pub operations: Vec<TransactionOperation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error: Option<String>,
}

impl Transaction {
    pub fn new() -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            status: TransactionStatus::Created,
            operations: Vec::new(),
            created_at: now,
            updated_at: now,
            error: None,
        }
    }

    pub fn add_operation(
        &mut self,
        operation: TransactionOperation,
    ) -> FoundationResult<()> {
        if self.status != TransactionStatus::Created {
            return Err(
                FoundationError::Transaction(
                    "cannot modify a non-created transaction"
                        .into(),
                ),
            );
        }

        self.operations.push(operation);
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn prepare(
        &mut self,
    ) -> FoundationResult<()> {
        if self.operations.is_empty() {
            return Err(
                FoundationError::Transaction(
                    "cannot prepare an empty transaction"
                        .into(),
                ),
            );
        }

        if self.status != TransactionStatus::Created {
            return Err(
                FoundationError::Transaction(
                    "transaction is not in created state"
                        .into(),
                ),
            );
        }

        self.status = TransactionStatus::Prepared;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn commit(
        &mut self,
    ) -> FoundationResult<()> {
        if self.status != TransactionStatus::Prepared {
            return Err(
                FoundationError::Transaction(
                    "transaction must be prepared before commit"
                        .into(),
                ),
            );
        }

        self.status = TransactionStatus::Committed;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn rollback(
        &mut self,
    ) -> FoundationResult<()> {
        match self.status {
            TransactionStatus::Committed
            | TransactionStatus::RolledBack => {
                return Err(
                    FoundationError::Transaction(
                        "transaction cannot be rolled back"
                            .into(),
                    ),
                );
            }

            _ => {}
        }

        self.status = TransactionStatus::RolledBack;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn fail(
        &mut self,
        error: impl Into<String>,
    ) {
        self.status = TransactionStatus::Failed;
        self.error = Some(error.into());
        self.updated_at = Utc::now();
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct TransactionManager {
    active: Arc<
        Mutex<
            std::collections::HashMap<
                Uuid,
                Transaction,
            >,
        >,
    >,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn begin(
        &self,
    ) -> Transaction {
        let transaction = Transaction::new();

        self.active
            .lock()
            .await
            .insert(
                transaction.id,
                transaction.clone(),
            );

        transaction
    }

    pub async fn update(
        &self,
        transaction: Transaction,
    ) -> FoundationResult<()> {
        let mut active = self.active.lock().await;

        if !active.contains_key(&transaction.id) {
            return Err(
                FoundationError::Transaction(
                    "transaction does not exist"
                        .into(),
                ),
            );
        }

        active.insert(
            transaction.id,
            transaction,
        );

        Ok(())
    }

    pub async fn get(
        &self,
        id: Uuid,
    ) -> Option<Transaction> {
        self.active.lock().await.get(&id).cloned()
    }

    pub async fn remove(
        &self,
        id: Uuid,
    ) -> Option<Transaction> {
        self.active.lock().await.remove(&id)
    }
}

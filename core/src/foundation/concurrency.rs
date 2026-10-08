use std::sync::Arc;

use tokio::sync::{Mutex, OwnedMutexGuard};

use super::errors::{
    FoundationError,
    FoundationResult,
};

#[derive(Clone)]
pub struct ConcurrencyGuard {
    semaphore: Arc<
        tokio::sync::Semaphore,
    >,
}

impl ConcurrencyGuard {
    pub fn new(max_concurrent: usize) -> FoundationResult<Self> {
        if max_concurrent == 0 {
            return Err(
                FoundationError::Concurrency(
                    "max concurrency must be greater than zero"
                        .into(),
                ),
            );
        }

        Ok(Self {
            semaphore: Arc::new(
                tokio::sync::Semaphore::new(
                    max_concurrent,
                ),
            ),
        })
    }

    pub async fn acquire(
        &self,
    ) -> FoundationResult<
        tokio::sync::OwnedSemaphorePermit,
    > {
        self.semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|error| {
                FoundationError::Concurrency(
                    error.to_string(),
                )
            })
    }

    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}

#[derive(Clone)]
pub struct ExecutionLock {
    lock: Arc<Mutex<()>>,
}

impl ExecutionLock {
    pub fn new() -> Self {
        Self {
            lock: Arc::new(Mutex::new(())),
        }
    }

    pub async fn acquire(
        &self,
    ) -> OwnedMutexGuard<()> {
        self.lock.clone().lock_owned().await
    }
}

impl Default for ExecutionLock {
    fn default() -> Self {
        Self::new()
    }
}

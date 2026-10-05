use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use super::errors::{FoundationError, FoundationResult};

#[derive(Debug, Clone, Default)]
pub struct ConcurrencyGuard {
    lock: Arc<RwLock<()>>,
}

impl ConcurrencyGuard {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn read(
        &self,
    ) -> FoundationResult<tokio::sync::RwLockReadGuard<'_, ()>> {
        Ok(self.lock.read().await)
    }

    pub async fn write(
        &self,
    ) -> FoundationResult<tokio::sync::RwLockWriteGuard<'_, ()>> {
        Ok(self.lock.write().await)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExecutionLock {
    lock: Arc<Mutex<()>>,
}

impl ExecutionLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn acquire(
        &self,
    ) -> FoundationResult<tokio::sync::MutexGuard<'_, ()>> {
        Ok(self.lock.lock().await)
    }
}

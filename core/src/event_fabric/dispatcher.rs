use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use super::envelope::EventEnvelope;

pub type EventHandlerId = String;

#[derive(Debug, Clone)]
pub enum DispatchOutcome {
    Handled,
    Ignored,
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("handler not found: {0}")]
    HandlerNotFound(String),

    #[error("handler failed: {0}")]
    HandlerFailed(String),
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    fn id(&self) -> EventHandlerId;

    async fn handle(
        &self,
        event: EventEnvelope,
    ) -> Result<DispatchOutcome, DispatchError>;
}

#[derive(Clone, Default)]
pub struct EventHandlerRegistry {
    handlers: Arc<RwLock<HashMap<EventHandlerId, Arc<dyn EventHandler>>>>,
}

impl EventHandlerRegistry {
    pub async fn register(
        &self,
        handler: Arc<dyn EventHandler>,
    ) {
        self.handlers
            .write()
            .await
            .insert(handler.id(), handler);
    }

    pub async fn remove(
        &self,
        id: &str,
    ) -> bool {
        self.handlers.write().await.remove(id).is_some()
    }

    pub async fn dispatch(
        &self,
        handler_id: &str,
        envelope: EventEnvelope,
    ) -> Result<DispatchOutcome, DispatchError> {
        let handler = self
            .handlers
            .read()
            .await
            .get(handler_id)
            .cloned()
            .ok_or_else(|| {
                DispatchError::HandlerNotFound(handler_id.to_string())
            })?;

        handler.handle(envelope).await
    }
}

use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::identity::identity::MidasIdentity;

use super::clock::MidasClock;
use super::contracts::{Event, EventKind};
use super::errors::{FoundationError, FoundationResult};
use super::event_bus::EventBus;
use super::lifecycle::{LifecycleController, LifecycleState};
use super::world_state::{SharedWorldState, WorldState};

#[derive(Debug, Clone)]
pub struct MidasRuntime {
    pub identity: MidasIdentity,
    pub clock: MidasClock,
    pub event_bus: EventBus,
    pub world_state: SharedWorldState,
    lifecycle: Arc<RwLock<LifecycleController>>,
}

impl MidasRuntime {
    pub fn new(identity: MidasIdentity) -> Self {
        Self {
            identity,
            clock: MidasClock::new(),
            event_bus: EventBus::new(),
            world_state: Arc::new(RwLock::new(WorldState::new())),
            lifecycle: Arc::new(RwLock::new(
                LifecycleController::new(),
            )),
        }
    }

    pub async fn start(&self) -> FoundationResult<()> {
        {
            let mut lifecycle = self.lifecycle.write().await;
            lifecycle.start()?;
        }

        self.publish(
            Event::new(
                EventKind::SystemStarted,
                "foundation.runtime",
                json!({
                    "system": "MIDAS",
                    "public_name": "Aeron Asford"
                }),
            ),
        )
        .await?;

        Ok(())
    }

    pub async fn heartbeat(&self) -> FoundationResult<()> {
        if !self.is_operational().await {
            return Err(FoundationError::RuntimeNotOperational);
        }

        self.publish(
            Event::new(
                EventKind::Heartbeat,
                "foundation.runtime",
                json!({
                    "timestamp": self.clock.now()
                }),
            ),
        )
        .await?;

        Ok(())
    }

    pub async fn stop(&self) -> FoundationResult<()> {
        {
            let mut lifecycle = self.lifecycle.write().await;
            lifecycle.stop()?;
        }

        self.publish(
            Event::new(
                EventKind::SystemStopped,
                "foundation.runtime",
                json!({
                    "system": "MIDAS"
                }),
            ),
        )
        .await?;

        Ok(())
    }

    pub async fn degrade(&self) {
        let mut lifecycle = self.lifecycle.write().await;
        lifecycle.degrade();
    }

    pub async fn lifecycle_state(&self) -> LifecycleState {
        self.lifecycle.read().await.state()
    }

    pub async fn is_operational(&self) -> bool {
        self.lifecycle.read().await.is_operational()
    }

    pub async fn can_accept_work(&self) -> bool {
        self.lifecycle.read().await.can_accept_work()
    }

    pub async fn publish(
        &self,
        event: Event,
    ) -> FoundationResult<()> {
        {
            let mut world = self.world_state.write().await;
            world.register_event(event.clone());
        }

        self.event_bus.publish(event)?;

        Ok(())
    }
}

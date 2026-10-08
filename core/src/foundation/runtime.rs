use std::sync::Arc;

use serde_json::json;

use super::{
    clock::MidasClock,
    contracts::{
        Event,
        EventKind,
    },
    errors::{
        FoundationError,
        FoundationResult,
    },
    event_bus::EventBus,
    lifecycle::{
        LifecycleController,
        LifecycleState,
    },
    world_state::{
        SharedWorldState,
        WorldState,
    },
};

#[derive(Debug, Clone)]
pub struct RuntimeHealth {
    pub lifecycle: LifecycleState,
    pub operational: bool,
    pub uptime_seconds: u64,
    pub event_subscribers: usize,
}

pub struct MidasRuntime {
    clock: MidasClock,
    lifecycle: LifecycleController,
    event_bus: EventBus,
    world_state: SharedWorldState,
}

impl MidasRuntime {
    pub fn new() -> Self {
        Self {
            clock: MidasClock::new(),
            lifecycle: LifecycleController::new(),
            event_bus: EventBus::default(),
            world_state: Arc::new(
                tokio::sync::RwLock::new(
                    WorldState::new(),
                ),
            ),
        }
    }

    pub async fn start(
        &self,
    ) -> FoundationResult<()> {
        self.lifecycle
            .transition(LifecycleState::Starting)
            .await?;

        self.lifecycle
            .transition(LifecycleState::Operational)
            .await?;

        let event = Event::new(
            EventKind::SystemStarted,
            "foundation.runtime",
            json!({
                "timestamp": self.clock.now(),
            }),
        );

        self.event_bus.publish(event)?;

        Ok(())
    }

    pub async fn stop(
        &self,
    ) -> FoundationResult<()> {
        let state = self.lifecycle.state().await;

        if state == LifecycleState::Stopped {
            return Ok(());
        }

        self.lifecycle
            .transition(LifecycleState::Stopping)
            .await?;

        let event = Event::new(
            EventKind::SystemStopping,
            "foundation.runtime",
            json!({
                "timestamp": self.clock.now(),
            }),
        );

        self.event_bus.publish(event)?;

        self.lifecycle
            .transition(LifecycleState::Stopped)
            .await?;

        let event = Event::new(
            EventKind::SystemStopped,
            "foundation.runtime",
            json!({
                "timestamp": self.clock.now(),
            }),
        );

        self.event_bus.publish(event)?;

        Ok(())
    }

    pub async fn heartbeat(
        &self,
    ) -> FoundationResult<()> {
        if !self.lifecycle.is_operational().await {
            return Err(
                FoundationError::RuntimeNotOperational
            );
        }

        let event = Event::new(
            EventKind::Heartbeat,
            "foundation.runtime",
            json!({
                "uptime_seconds":
                    self.clock.uptime_seconds(),
                "timestamp": self.clock.now(),
            }),
        );

        self.event_bus.publish(event)?;

        Ok(())
    }

    pub async fn require_operational(
        &self,
    ) -> FoundationResult<()> {
        if self.lifecycle.is_operational().await {
            Ok(())
        } else {
            match self.lifecycle.state().await {
                LifecycleState::Stopping => {
                    Err(FoundationError::RuntimeStopping)
                }

                LifecycleState::Stopped => {
                    Err(FoundationError::SystemStopped)
                }

                _ => {
                    Err(
                        FoundationError::RuntimeNotOperational
                    )
                }
            }
        }
    }

    pub fn clock(&self) -> &MidasClock {
        &self.clock
    }

    pub fn lifecycle(
        &self,
    ) -> &LifecycleController {
        &self.lifecycle
    }

    pub fn event_bus(
        &self,
    ) -> &EventBus {
        &self.event_bus
    }

    pub fn world_state(
        &self,
    ) -> &SharedWorldState {
        &self.world_state
    }

    pub async fn health(
        &self,
    ) -> RuntimeHealth {
        RuntimeHealth {
            lifecycle: self.lifecycle.state().await,
            operational: self.lifecycle.is_operational().await,
            uptime_seconds: self.clock.uptime_seconds(),
            event_subscribers:
                self.event_bus.receiver_count(),
        }
    }
}

impl Default for MidasRuntime {
    fn default() -> Self {
        Self::new()
    }
}

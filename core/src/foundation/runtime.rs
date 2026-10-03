use anyhow::Result;
use tracing::info;

use super::{
    clock::MidasClock,
    contracts::{Event, EventKind},
    event_bus::EventBus,
    world_state::WorldState,
};
use crate::identity::identity::MidasIdentity;

pub struct MidasRuntime {
    pub identity: MidasIdentity,
    pub clock: MidasClock,
    pub event_bus: EventBus,
    pub world_state: WorldState,
}

impl Default for MidasRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl MidasRuntime {
    pub fn new() -> Self {
        Self {
            identity: MidasIdentity::default(),
            clock: MidasClock,
            event_bus: EventBus::new(),
            world_state: WorldState::default(),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.identity.set_state(
            crate::identity::identity::IdentityState::Operational,
        );

        let event = Event::new(
            EventKind::SystemStarted,
            "midas-core",
            serde_json::json!({
                "system": self.identity.system_name,
                "public_name": self.identity.public_name,
                "version": self.identity.version,
            }),
        );

        self.event_bus.publish(event)?;

        info!(
            system = %self.identity.system_name,
            public_name = %self.identity.public_name,
            version = %self.identity.version,
            "MIDAS runtime started"
        );

        Ok(())
    }

    pub fn heartbeat(&mut self) -> Result<()> {
        let event = Event::new(
            EventKind::Heartbeat,
            "midas-core",
            serde_json::json!({
                "timestamp": self.clock.now(),
                "state": format!("{:?}", self.identity.state),
            }),
        );

        self.event_bus.publish(event)?;

        Ok(())
    }
}

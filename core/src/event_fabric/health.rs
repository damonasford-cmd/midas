use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventFabricHealthState {
    Healthy,
    Degraded,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFabricHealth {
    pub state: EventFabricHealthState,
    pub queued_events: usize,
    pub stored_events: usize,
    pub dead_letters: usize,
    pub duplicate_events: usize,
    pub failed_dispatches: usize,
}

impl Default for EventFabricHealth {
    fn default() -> Self {
        Self {
            state: EventFabricHealthState::Healthy,
            queued_events: 0,
            stored_events: 0,
            dead_letters: 0,
            duplicate_events: 0,
            failed_dispatches: 0,
        }
    }
}

use tokio::sync::broadcast;

use super::contracts::Event;
use super::errors::{FoundationError, FoundationResult};

const EVENT_BUFFER_SIZE: usize = 4096;

#[derive(Debug, Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(EVENT_BUFFER_SIZE);

        Self { sender }
    }

    pub fn publish(&self, event: Event) -> FoundationResult<usize> {
        self.sender
            .send(event)
            .map_err(|error| FoundationError::EventBus(error.to_string()))
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

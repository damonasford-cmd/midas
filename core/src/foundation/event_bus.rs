use anyhow::Result;
use tokio::sync::broadcast;

use super::contracts::Event;

const EVENT_BUFFER: usize = 4096;

#[derive(Clone)]
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
        let (sender, _) = broadcast::channel(EVENT_BUFFER);

        Self { sender }
    }

    pub fn publish(&self, event: Event) -> Result<usize> {
        match self.sender.send(event) {
            Ok(count) => Ok(count),
            Err(_) => Ok(0),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

use tokio::sync::broadcast;

use super::{
    contracts::Event,
    errors::{
        FoundationError,
        FoundationResult,
    },
};

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

pub struct EventSubscription {
    receiver: broadcast::Receiver<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);

        Self { sender }
    }

    pub fn publish(
        &self,
        event: Event,
    ) -> FoundationResult<usize> {
        self.sender
            .send(event)
            .map_err(|error| {
                FoundationError::EventBus(error.to_string())
            })
    }

    pub fn subscribe(&self) -> EventSubscription {
        EventSubscription {
            receiver: self.sender.subscribe(),
        }
    }

    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl EventSubscription {
    pub async fn recv(
        &mut self,
    ) -> FoundationResult<Event> {
        self.receiver
            .recv()
            .await
            .map_err(|error| {
                FoundationError::EventBus(error.to_string())
            })
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(4096)
    }
}

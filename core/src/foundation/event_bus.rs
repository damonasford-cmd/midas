
use tokio::sync::broadcast;

use super::{
    contracts::Event,
    errors::{FoundationError, FoundationResult},
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
        let (sender, _) = broadcast::channel(capacity.max(1));
        Self { sender }
    }

    /// Publie aux abonnés actifs.
    /// Zéro abonné signifie zéro livraison, mais pas une erreur système.
    pub fn publish(&self, event: Event) -> FoundationResult<usize> {
        match self.sender.send(event) {
            Ok(count) => Ok(count),
            Err(_) if self.sender.receiver_count() == 0 => Ok(0),
            Err(error) => Err(FoundationError::EventBus(error.to_string())),
        }
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
    pub async fn recv(&mut self) -> FoundationResult<Event> {
        match self.receiver.recv().await {
            Ok(event) => Ok(event),
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                Err(FoundationError::EventBus(format!(
                    "subscription lagged; {skipped} event(s) were skipped"
                )))
            }
            Err(broadcast::error::RecvError::Closed) => {
                Err(FoundationError::EventBus(
                    "event channel closed".into(),
                ))
            }
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(4096)
    }
}

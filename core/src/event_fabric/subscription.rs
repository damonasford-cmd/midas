use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::topic::TopicPattern;

pub type SubscriptionId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    pub id: SubscriptionId,
    pub subscriber: String,
    pub pattern: TopicPattern,
    pub enabled: bool,
}

impl EventSubscription {
    pub fn new(
        subscriber: impl Into<String>,
        pattern: TopicPattern,
    ) -> Result<Self, String> {
        let subscriber = subscriber.into();

        if subscriber.trim().is_empty() {
            return Err("subscriber cannot be empty".into());
        }

        Ok(Self {
            id: Uuid::new_v4(),
            subscriber,
            pattern,
            enabled: true,
        })
    }
}

#[derive(Debug, Default)]
pub struct SubscriptionRegistry {
    subscriptions: Vec<EventSubscription>,
}

impl SubscriptionRegistry {
    pub fn register(&mut self, subscription: EventSubscription) {
        self.subscriptions.push(subscription);
    }

    pub fn remove(&mut self, id: SubscriptionId) -> bool {
        let before = self.subscriptions.len();

        self.subscriptions.retain(|item| item.id != id);

        before != self.subscriptions.len()
    }

    pub fn matching(
        &self,
        topic: &super::topic::EventTopic,
    ) -> Vec<EventSubscription> {
        self.subscriptions
            .iter()
            .filter(|subscription| {
                subscription.enabled && topic.matches(&subscription.pattern)
            })
            .cloned()
            .collect()
    }

    pub fn all(&self) -> &[EventSubscription] {
        &self.subscriptions
    }
}

use serde::{Deserialize, Serialize};

use super::{
    envelope::EventEnvelope,
    subscription::SubscriptionRegistry,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRoute {
    pub subscriber: String,
    pub event_id: uuid::Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct RoutingDecision {
    pub routes: Vec<EventRoute>,
}

#[derive(Debug, Default)]
pub struct EventRouter;

impl EventRouter {
    pub fn route(
        &self,
        envelope: &EventEnvelope,
        subscriptions: &SubscriptionRegistry,
    ) -> RoutingDecision {
        let routes = subscriptions
            .matching(&envelope.topic)
            .into_iter()
            .map(|subscription| EventRoute {
                subscriber: subscription.subscriber,
                event_id: envelope.id(),
            })
            .collect();

        RoutingDecision { routes }
    }
}

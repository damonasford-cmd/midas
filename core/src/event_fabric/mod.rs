pub mod context;
pub mod dead_letter;
pub mod deduplication;
pub mod dispatcher;
pub mod envelope;
pub mod event;
pub mod fabric;
pub mod health;
pub mod metrics;
pub mod ordering;
pub mod priority;
pub mod retry;
pub mod router;
pub mod store;
pub mod subscription;
pub mod topic;

pub use context::{EventContext, EventContextBuilder};

pub use dead_letter::{
    DeadLetterEntry,
    DeadLetterQueue,
};

pub use deduplication::{
    DeduplicationDecision,
    DeduplicationStore,
};

pub use dispatcher::{
    DispatchError,
    DispatchOutcome,
    EventHandler,
    EventHandlerId,
    EventHandlerRegistry,
};

pub use envelope::{
    EventEnvelope,
    EventEnvelopeBuilder,
};

pub use event::{
    EventId,
    FabricEvent,
};

pub use fabric::{
    EventFabric,
    EventFabricConfig,
    EventFabricState,
};

pub use health::{
    EventFabricHealth,
    EventFabricHealthState,
};

pub use metrics::{
    EventFabricMetrics,
    EventMetricSnapshot,
};

pub use ordering::{
    EventOrderingKey,
    OrderingDecision,
    OrderingStore,
};

pub use priority::EventPriority;

pub use retry::{
    RetryDecision,
    RetryPolicy,
};

pub use router::{
    EventRoute,
    EventRouter,
    RoutingDecision,
};

pub use store::{
    EventStore,
    StoredEvent,
};

pub use subscription::{
    EventSubscription,
    SubscriptionId,
    SubscriptionRegistry,
};

pub use topic::{
    EventTopic,
    TopicPattern,
};

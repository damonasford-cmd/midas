use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    context::EventContext,
    event::{EventId, FabricEvent},
    priority::EventPriority,
    topic::EventTopic,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub envelope_id: Uuid,
    pub event: FabricEvent,

    pub topic: EventTopic,

    pub source: String,
    pub destination: Option<String>,

    pub correlation_id: Uuid,
    pub causation_id: Option<EventId>,

    pub priority: EventPriority,

    pub created_at: DateTime<Utc>,

    pub attempt: u32,

    pub context: EventContext,
}

impl EventEnvelope {
    pub fn builder(
        event: FabricEvent,
        topic: EventTopic,
        source: impl Into<String>,
    ) -> EventEnvelopeBuilder {
        EventEnvelopeBuilder {
            event,
            topic,
            source: source.into(),
            destination: None,
            correlation_id: Uuid::new_v4(),
            causation_id: None,
            priority: EventPriority::Normal,
            context: EventContext::default(),
        }
    }

    pub fn id(&self) -> EventId {
        self.event.id
    }

    pub fn retry(&self) -> Self {
        let mut next = self.clone();
        next.attempt = next.attempt.saturating_add(1);
        next
    }
}

pub struct EventEnvelopeBuilder {
    event: FabricEvent,
    topic: EventTopic,
    source: String,
    destination: Option<String>,
    correlation_id: Uuid,
    causation_id: Option<EventId>,
    priority: EventPriority,
    context: EventContext,
}

impl EventEnvelopeBuilder {
    pub fn destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }

    pub fn correlation_id(mut self, id: Uuid) -> Self {
        self.correlation_id = id;
        self
    }

    pub fn causation_id(mut self, id: EventId) -> Self {
        self.causation_id = Some(id);
        self
    }

    pub fn priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn context(mut self, context: EventContext) -> Self {
        self.context = context;
        self
    }

    pub fn build(self) -> Result<EventEnvelope, String> {
        self.event.validate()?;

        if self.source.trim().is_empty() {
            return Err("event source cannot be empty".into());
        }

        Ok(EventEnvelope {
            envelope_id: Uuid::new_v4(),
            event: self.event,
            topic: self.topic,
            source: self.source,
            destination: self.destination,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
            priority: self.priority,
            created_at: Utc::now(),
            attempt: 0,
            context: self.context,
        })
    }
}

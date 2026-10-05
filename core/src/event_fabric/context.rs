use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContext {
    pub actor: Option<String>,
    pub session_id: Option<Uuid>,
    pub request_id: Option<Uuid>,
    pub authorization_context: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Default for EventContext {
    fn default() -> Self {
        Self {
            actor: None,
            session_id: None,
            request_id: None,
            authorization_context: None,
            metadata: HashMap::new(),
        }
    }
}

impl EventContext {
    pub fn builder() -> EventContextBuilder {
        EventContextBuilder::default()
    }
}

#[derive(Default)]
pub struct EventContextBuilder {
    actor: Option<String>,
    session_id: Option<Uuid>,
    request_id: Option<Uuid>,
    authorization_context: Option<String>,
    metadata: HashMap<String, String>,
}

impl EventContextBuilder {
    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: Uuid) -> Self {
        self.session_id = Some(value);
        self
    }

    pub fn request_id(mut self, value: Uuid) -> Self {
        self.request_id = Some(value);
        self
    }

    pub fn authorization_context(mut self, value: impl Into<String>) -> Self {
        self.authorization_context = Some(value.into());
        self
    }

    pub fn metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> EventContext {
        EventContext {
            actor: self.actor,
            session_id: self.session_id,
            request_id: self.request_id,
            authorization_context: self.authorization_context,
            metadata: self.metadata,
        }
    }
}

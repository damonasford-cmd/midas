use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventTopic(pub String);

impl EventTopic {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err("event topic cannot be empty".into());
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn matches(&self, pattern: &TopicPattern) -> bool {
        match pattern {
            TopicPattern::Exact(value) => self.0 == *value,
            TopicPattern::Prefix(prefix) => self.0.starts_with(prefix),
            TopicPattern::Any => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TopicPattern {
    Exact(String),
    Prefix(String),
    Any,
}

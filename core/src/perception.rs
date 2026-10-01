use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerceptionKind {
    Text,
    Data,
    Image,
    Audio,
    Video,
    File,
    Software,
    Machine,
    Sensor,
    Event,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perception {
    pub kind: PerceptionKind,
    pub source: String,
    pub content: String,
}

impl Perception {
    pub fn new(
        kind: PerceptionKind,
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source: source.into(),
            content: content.into(),
        }
    }
}

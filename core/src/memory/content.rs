use serde::{Deserialize, Serialize};

use super::modality::MemoryModality;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContentKind {
    Text(String),

    BinaryReference {
        uri: String,
        mime_type: Option<String>,
        size_bytes: Option<u64>,
    },

    Structured(serde_json::Value),

    Event {
        event_type: String,
        payload: serde_json::Value,
    },

    Observation {
        modality: MemoryModality,
        representation: serde_json::Value,
    },

    Action {
        action_type: String,
        representation: serde_json::Value,
    },

    Mixed(Vec<MemoryContentKind>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryContent {
    pub modality: MemoryModality,
    pub kind: MemoryContentKind,
}

impl MemoryContent {
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            modality: MemoryModality::Text,
            kind: MemoryContentKind::Text(value.into()),
        }
    }

    pub fn structured(value: serde_json::Value) -> Self {
        Self {
            modality: MemoryModality::Structured,
            kind: MemoryContentKind::Structured(value),
        }
    }

    pub fn event(
        event_type: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            modality: MemoryModality::Event,
            kind: MemoryContentKind::Event {
                event_type: event_type.into(),
                payload,
            },
        }
    }

    pub fn observation(
        modality: MemoryModality,
        representation: serde_json::Value,
    ) -> Self {
        Self {
            modality: MemoryModality::Observation,
            kind: MemoryContentKind::Observation {
                modality,
                representation,
            },
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        match &self.kind {
            MemoryContentKind::Text(value) => {
                if value.trim().is_empty() {
                    return Err("memory text cannot be empty".into());
                }
            }

            MemoryContentKind::BinaryReference { uri, .. } => {
                if uri.trim().is_empty() {
                    return Err("memory binary reference cannot be empty".into());
                }
            }

            MemoryContentKind::Mixed(values) => {
                if values.is_empty() {
                    return Err(
                        "mixed memory content cannot be empty".into()
                    );
                }
            }

            _ => {}
        }

        Ok(())
    }
}

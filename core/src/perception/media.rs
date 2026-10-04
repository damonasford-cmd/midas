use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaObservation {
    pub id: Uuid,
    pub source: String,
    pub media_type: MediaType,
    pub description: String,
    pub extracted_information: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Text,
    Image,
    Audio,
    Video,
    Document,
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct MediaPerception;

impl MediaPerception {
    pub fn new() -> Self {
        Self
    }

    pub fn observe(
        &self,
        source: impl Into<String>,
        media_type: MediaType,
        description: impl Into<String>,
    ) -> MediaObservation {
        MediaObservation {
            id: Uuid::new_v4(),
            source: source.into(),
            media_type,
            description: description.into(),
            extracted_information: Vec::new(),
            confidence: 0.0,
        }
    }
}

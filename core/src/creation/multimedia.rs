use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Text,
    Image,
    Audio,
    Video,
    Music,
    Voice,
    Animation,
    ThreeD,
    Interactive,
    Code,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimediaRequest {
    pub id: Uuid,
    pub media_type: MediaType,
    pub objective: String,
    pub requirements: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimediaArtifact {
    pub id: Uuid,
    pub request_id: Uuid,
    pub media_type: MediaType,
    pub location: Option<String>,
    pub description: String,
    pub quality_score: Option<f64>,
    pub validated: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MultimediaCreator;

impl MultimediaCreator {
    pub fn new() -> Self {
        Self
    }

    pub fn request(
        &self,
        media_type: MediaType,
        objective: impl Into<String>,
    ) -> MultimediaRequest {
        MultimediaRequest {
            id: Uuid::new_v4(),
            media_type,
            objective: objective.into(),
            requirements: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn register_artifact(
        &self,
        request: &MultimediaRequest,
        description: impl Into<String>,
        location: Option<String>,
    ) -> MultimediaArtifact {
        MultimediaArtifact {
            id: Uuid::new_v4(),
            request_id: request.id,
            media_type: request.media_type.clone(),
            location,
            description: description.into(),
            quality_score: None,
            validated: false,
        }
    }

    pub fn validate(
        &self,
        artifact: &mut MultimediaArtifact,
        quality_score: f64,
    ) {
        artifact.quality_score = Some(quality_score.clamp(0.0, 1.0));
        artifact.validated = quality_score >= 0.8;
    }
}

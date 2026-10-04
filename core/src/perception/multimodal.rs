use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalInput {
    pub id: Uuid,
    pub text: Option<String>,
    pub image_refs: Vec<String>,
    pub audio_refs: Vec<String>,
    pub video_refs: Vec<String>,
    pub file_refs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct MultimodalPerception;

impl MultimodalPerception {
    pub fn new() -> Self {
        Self
    }

    pub fn create(&self) -> MultimodalInput {
        MultimodalInput {
            id: Uuid::new_v4(),
            text: None,
            image_refs: Vec::new(),
            audio_refs: Vec::new(),
            video_refs: Vec::new(),
            file_refs: Vec::new(),
        }
    }
}

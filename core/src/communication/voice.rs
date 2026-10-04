use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInput {
    pub id: Uuid,
    pub source: String,
    pub transcript: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceOutput {
    pub id: Uuid,
    pub text: String,
    pub voice: String,
    pub audio_location: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VoiceEngine;

impl VoiceEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn input(
        &self,
        source: impl Into<String>,
        transcript: impl Into<String>,
        confidence: f64,
    ) -> VoiceInput {
        VoiceInput {
            id: Uuid::new_v4(),
            source: source.into(),
            transcript: transcript.into(),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    pub fn output(
        &self,
        text: impl Into<String>,
        voice: impl Into<String>,
    ) -> VoiceOutput {
        VoiceOutput {
            id: Uuid::new_v4(),
            text: text.into(),
            voice: voice.into(),
            audio_location: None,
        }
    }
}

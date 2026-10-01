use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider {
    Ollama,
    Vllm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub model: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OllamaResponse {
    pub model: String,
    pub message: ChatMessage,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VllmResponse {
    pub model: String,
    pub choices: Vec<VllmChoice>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VllmChoice {
    pub message: ChatMessage,
}

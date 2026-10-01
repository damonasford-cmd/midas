use anyhow::{anyhow, Context, Result};
use reqwest::Client;

use super::types::{
    ChatRequest,
    ChatResponse,
    ModelProvider,
    OllamaResponse,
    VllmResponse,
};

pub struct ModelClient {
    http: Client,
    provider: ModelProvider,
    base_url: String,
}

impl ModelClient {
    pub fn new(
        provider: ModelProvider,
        base_url: impl Into<String>,
    ) -> Result<Self> {
        let http = Client::builder()
            .build()
            .context("Impossible de créer le client HTTP du Model Gateway")?;

        Ok(Self {
            http,
            provider,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        })
    }

    pub async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse> {
        match self.provider {
            ModelProvider::Ollama => self.chat_ollama(request).await,
            ModelProvider::Vllm => self.chat_vllm(request).await,
        }
    }

    async fn chat_ollama(&self, request: &ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/api/chat", self.base_url);

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "stream": false,
            "options": {
                "temperature": request.temperature.unwrap_or(0.2)
            }
        });

        let response = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await
            .context("Erreur de connexion à Ollama")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Ollama a répondu avec le statut {}",
                response.status()
            ));
        }

        let data: OllamaResponse = response
            .json()
            .await
            .context("Réponse Ollama invalide")?;

        Ok(ChatResponse {
            model: data.model,
            content: data.message.content,
        })
    }

    async fn chat_vllm(&self, request: &ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/v1/chat/completions", self.base_url);

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.2),
            "stream": false
        });

        let response = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await
            .context("Erreur de connexion à vLLM")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "vLLM a répondu avec le statut {}",
                response.status()
            ));
        }

        let data: VllmResponse = response
            .json()
            .await
            .context("Réponse vLLM invalide")?;

        let choice = data
            .choices
            .first()
            .ok_or_else(|| anyhow!("vLLM n'a retourné aucun choix"))?;

        Ok(ChatResponse {
            model: data.model,
            content: choice.message.content.clone(),
        })
    }
}

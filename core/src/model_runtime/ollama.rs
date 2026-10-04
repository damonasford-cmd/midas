use crate::model_runtime::provider::{
    ModelMessage,
    ModelProvider,
    ModelRequest,
    ModelResponse,
};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct OllamaProvider {
    client: Client,
    base_url: String,
}

impl OllamaProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    messages: &'a [ModelMessage],
    stream: bool,
    temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    model: String,
    message: ModelMessage,
    #[allow(dead_code)]
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<u64>,
    #[serde(default)]
    eval_count: Option<u64>,
}

#[async_trait]
impl ModelProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn chat(
        &self,
        request: &ModelRequest,
    ) -> Result<ModelResponse> {
        let url = format!("{}/api/chat", self.base_url);

        let response: OllamaResponse = self
            .client
            .post(url)
            .json(&OllamaRequest {
                model: &request.model,
                messages: &request.messages,
                stream: false,
                temperature: request.temperature,
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(ModelResponse {
            model: response.model,
            content: response.message.content,
            input_tokens: response.prompt_eval_count,
            output_tokens: response.eval_count,
        })
    }

    async fn health(&self) -> Result<bool> {
        Ok(self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?
            .status()
            .is_success())
    }

    async fn models(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Tags {
            models: Vec<Model>,
        }

        #[derive(Deserialize)]
        struct Model {
            name: String,
        }

        let result: Tags = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(result.models.into_iter().map(|m| m.name).collect())
    }
}

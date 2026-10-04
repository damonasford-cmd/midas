use crate::model_runtime::provider::{
    ModelMessage,
    ModelProvider,
    ModelRequest,
    ModelResponse,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct VllmProvider {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl VllmProvider {
    pub fn new(
        base_url: impl Into<String>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key,
        }
    }

    fn request_builder(
        &self,
        request: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        match &self.api_key {
            Some(key) => request.bearer_auth(key),
            None => request,
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [ModelMessage],
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    model: String,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ModelMessage,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
}

#[async_trait]
impl ModelProvider for VllmProvider {
    fn name(&self) -> &str {
        "vllm"
    }

    async fn chat(
        &self,
        request: &ModelRequest,
    ) -> Result<ModelResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let http_request = self.client.post(url);

        let response: ChatResponse = self
            .request_builder(http_request)
            .json(&ChatRequest {
                model: &request.model,
                messages: &request.messages,
                stream: false,
                temperature: request.temperature,
                max_tokens: request.max_tokens,
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("vLLM returned no choices"))?;

        Ok(ModelResponse {
            model: response.model,
            content: choice.message.content,
            input_tokens: response
                .usage
                .as_ref()
                .and_then(|usage| usage.prompt_tokens),
            output_tokens: response
                .usage
                .as_ref()
                .and_then(|usage| usage.completion_tokens),
        })
    }

    async fn health(&self) -> Result<bool> {
        let response = self
            .request_builder(
                self.client.get(format!("{}/models", self.base_url)),
            )
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    async fn models(&self) -> Result<Vec<String>> {
        #[derive(Debug, Deserialize)]
        struct Models {
            data: Vec<Model>,
        }

        #[derive(Debug, Deserialize)]
        struct Model {
            id: String,
        }

        let result: Models = self
            .request_builder(
                self.client.get(format!("{}/models", self.base_url)),
            )
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(result.data.into_iter().map(|m| m.id).collect())
    }
}

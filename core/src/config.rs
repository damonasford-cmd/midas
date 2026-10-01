use anyhow::{Context, Result};
use std::env;

use crate::model::ModelProvider;

#[derive(Debug, Clone)]
pub struct MidasConfig {
    pub model_provider: ModelProvider,
    pub model_url: String,
    pub model_name: String,
    pub model_temperature: f32,
    pub memory_path: String,
}

impl MidasConfig {
    pub fn from_env() -> Result<Self> {
        let provider = env::var("MIDAS_MODEL_PROVIDER")
            .unwrap_or_else(|_| "ollama".to_string());

        let model_provider = match provider.to_lowercase().as_str() {
            "ollama" => ModelProvider::Ollama,

            "vllm" => ModelProvider::Vllm,

            other => {
                anyhow::bail!(
                    "Provider de modèle inconnu : {other}"
                );
            }
        };

        let model_url = env::var("MIDAS_MODEL_URL")
            .unwrap_or_else(|_| {
                "http://127.0.0.1:11434".to_string()
            });

        let model_name = env::var("MIDAS_MODEL_NAME")
            .unwrap_or_else(|_| {
                "llama3.3:70b".to_string()
            });

        let model_temperature = env::var("MIDAS_MODEL_TEMPERATURE")
            .unwrap_or_else(|_| "0.2".to_string())
            .parse::<f32>()
            .context(
                "MIDAS_MODEL_TEMPERATURE invalide",
            )?;

        let memory_path = env::var("MIDAS_MEMORY_PATH")
            .unwrap_or_else(|_| {
                "data/memory/midas.memory".to_string()
            });

        Ok(Self {
            model_provider,
            model_url,
            model_name,
            model_temperature,
            memory_path,
        })
    }
}

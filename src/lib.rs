//! MODEL GATEWAY
//! Passerelle vers les moteurs d'inférence locaux.
//! Ollama et vLLM uniquement.

use serde::{Deserialize, Serialize};
use tracing::info;

/// Les moteurs d'inférence supportés.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Moteur {
    Ollama,
    Vllm,
}

/// Configuration du Model Gateway.
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub moteur: Moteur,
    pub url: String,
    pub modele: String,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            moteur: Moteur::Ollama,
            url: "http://127.0.0.1:11434".to_string(),
            modele: "llama3.3:70b".to_string(),
        }
    }
}

/// Passerelle vers le moteur de modèle.
pub struct ModelGateway {
    pub config: GatewayConfig,
    client: reqwest::Client,
}

impl ModelGateway {
    /// Crée un nouveau Model Gateway.
    pub fn new(config: GatewayConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Envoie un prompt au moteur sélectionné
    /// et retourne sa réponse.
    pub async fn generer(&self, prompt: &str) -> anyhow::Result<String> {
        match self.config.moteur {
            Moteur::Ollama => self.appeler_ollama(prompt).await,
            Moteur::Vllm => self.appeler_vllm(prompt).await,
        }
    }

    /// Appel du serveur Ollama.
    async fn appeler_ollama(
        &self,
        prompt: &str,
    ) -> anyhow::Result<String> {
        info!("Model Gateway: appel Ollama");

        let url = format!(
            "{}/api/generate",
            self.config.url.trim_end_matches('/')
        );

        let payload = serde_json::json!({
            "model": self.config.modele,
            "prompt": prompt,
            "stream": false
        });

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let json: serde_json::Value = resp.json().await?;

        let reponse = json["response"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(reponse)
    }

    /// Appel du serveur vLLM.
    async fn appeler_vllm(
        &self,
        prompt: &str,
    ) -> anyhow::Result<String> {
        info!("Model Gateway: appel vLLM");

        let url = format!(
            "{}/v1/completions",
            self.config.url.trim_end_matches('/')
        );

        let payload = serde_json::json!({
            "model": self.config.modele,
            "prompt": prompt,
            "max_tokens": 1024
        });

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let json: serde_json::Value = resp.json().await?;

        let reponse = json["choices"]
            .get(0)
            .and_then(|choice| choice["text"].as_str())
            .unwrap_or("")
            .to_string();

        Ok(reponse)
    }
}

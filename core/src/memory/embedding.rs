use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub dimensions: usize,
    pub values: Vec<f32>,
    pub model: String,
}

impl Embedding {
    pub fn validate(&self) -> Result<(), String> {
        if self.dimensions == 0 {
            return Err("embedding dimensions cannot be zero".into());
        }

        if self.values.len() != self.dimensions {
            return Err(
                "embedding dimensions do not match values length".into()
            );
        }

        if self.model.trim().is_empty() {
            return Err("embedding model cannot be empty".into());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingReference {
    pub provider: String,
    pub model: String,
    pub dimensions: usize,
}

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    fn provider_name(&self) -> &str;

    async fn embed(
        &self,
        content: &str,
    ) -> Result<Embedding, String>;
}

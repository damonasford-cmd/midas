use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebResource {
    pub url: String,
    pub title: Option<String>,
    pub content: String,
    pub status_code: u16,
}

#[derive(Clone)]
pub struct WebPerception {
    client: Client,
}

impl Default for WebPerception {
    fn default() -> Self {
        Self::new()
    }
}

impl WebPerception {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<WebResource> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(anyhow!("unsupported URL scheme"));
        }

        let response = self.client.get(url).send().await?;
        let status_code = response.status().as_u16();
        let final_url = response.url().to_string();
        let content = response.text().await?;

        Ok(WebResource {
            url: final_url,
            title: None,
            content,
            status_code,
        })
    }
}

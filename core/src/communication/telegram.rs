use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub update_id: i64,
    pub chat_id: i64,
    pub message_id: i64,
    pub text: Option<String>,
}

#[derive(Clone)]
pub struct TelegramClient {
    client: Client,
    base_url: String,
    bot_token: String,
}

impl TelegramClient {
    pub fn new(bot_token: impl Into<String>) -> Self {
        let token = bot_token.into();

        Self {
            client: Client::new(),
            base_url: "https://api.telegram.org".to_string(),
            bot_token: token,
        }
    }

    fn validate(&self) -> Result<()> {
        if self.bot_token.trim().is_empty() {
            return Err(anyhow!("Telegram bot token is empty"));
        }

        Ok(())
    }

    pub async fn send_message(
        &self,
        chat_id: i64,
        text: &str,
    ) -> Result<()> {
        self.validate()?;

        let url = format!(
            "{}/bot{}/sendMessage",
            self.base_url,
            self.bot_token
        );

        self.client
            .post(url)
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": text
            }))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

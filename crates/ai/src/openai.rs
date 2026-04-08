use async_trait::async_trait;

use crate::client::chat;
use crate::model::*;

const BASE_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Default)]
pub struct OpenAIClient {
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl chat::ChatCompletionClient for OpenAIClient {
    async fn chat(
        &self,
        access_token: &str,
        req: chat::ChatCompletionRequest,
    ) -> Result<chat::ChatCompletionResponse, ModelError> {
        let res = self
            .client
            .post(BASE_URL)
            .bearer_auth(access_token)
            .json(&req)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status().as_u16();
            let body = res.text().await.unwrap_or_default();
            return Err(ModelError::Api {
                status,
                message: body,
            });
        }

        let response = res.json::<chat::ChatCompletionResponse>().await?;
        Ok(response)
    }
}

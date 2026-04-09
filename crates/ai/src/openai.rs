use async_trait::async_trait;

use crate::Error;
use crate::client::chat;

const BASE_URL: &str = "https://api.openai.com/v1/chat/completions";

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

impl Default for OpenAIClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl chat::ChatCompletionClient for OpenAIClient {
    async fn chat(
        &self,
        access_token: &str,
        req: chat::ChatCompletionRequest,
    ) -> Result<chat::ChatCompletionResponse, Error> {
        let res = self
            .client
            .post(BASE_URL)
            .bearer_auth(access_token)
            .json(&req)
            .send()
            .await?;

        match res.error_for_status() {
            Err(err) => Err(Error::Http(err)),
            Ok(res) => Ok(res.json::<chat::ChatCompletionResponse>().await?),
        }
    }
}

use async_trait::async_trait;

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

impl Model for OpenAIClient {
    fn id(&self) -> ModelId {
        ModelId {
            provider: ProviderId::OpenAI,
            id: "chat".to_string(),
        }
    }
}

#[async_trait]
impl ChatCompletionModel for OpenAIClient {
    async fn chat(
        &self,
        access_token: &str,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ModelError> {
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

        let response = res.json::<ChatCompletionResponse>().await?;
        Ok(response)
    }
}

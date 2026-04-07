use async_trait::async_trait;

use crate::model::{Model, ModelError};

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionUsage {
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub access_token: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionResponse {
    pub message: Message,
    pub usage: Option<ChatCompletionUsage>,
}

#[async_trait]
pub trait ChatCompletionModel: Model {
    async fn chat(&self, req: ChatCompletionRequest) -> Result<ChatCompletionResponse, ModelError>;
}

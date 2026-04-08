mod format;
mod message;
mod tool;

pub use format::*;
pub use message::*;
pub use tool::*;

use async_trait::async_trait;
use serde_with::skip_serializing_none;

use crate::model::ModelError;

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    pub max_completion_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub stop: Option<Vec<String>>,
    pub tools: Option<Vec<ChatCompletionTool>>,
    pub tool_choice: Option<ChatCompletionToolChoice>,
    pub response_format: Option<ResponseFormat>,
    pub reasoning_effort: Option<ReasoningEffort>,
    pub seed: Option<u64>,
    pub n: Option<u32>,
    pub store: Option<bool>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Option<CompletionUsage>,
    pub system_fingerprint: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatCompletionMessage,
    pub finish_reason: Option<FinishReason>,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PromptTokensDetails {
    pub cached_tokens: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: Option<u32>,
    pub accepted_prediction_tokens: Option<u32>,
    pub rejected_prediction_tokens: Option<u32>,
}

impl ChatCompletionRequest {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            messages: Vec::new(),
            max_completion_tokens: None,
            temperature: None,
            top_p: None,
            stop: None,
            tools: None,
            tool_choice: None,
            response_format: None,
            reasoning_effort: None,
            seed: None,
            n: None,
            store: None,
            frequency_penalty: None,
            presence_penalty: None,
        }
    }

    pub fn with_messages(mut self, messages: Vec<ChatCompletionMessage>) -> Self {
        self.messages = messages;
        self
    }

    pub fn with_max_completion_tokens(mut self, max_completion_tokens: u32) -> Self {
        self.max_completion_tokens = Some(max_completion_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn with_tools(mut self, tools: Vec<ChatCompletionTool>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn with_tool_choice(mut self, tool_choice: ChatCompletionToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    pub fn with_response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn with_reasoning_effort(mut self, reasoning_effort: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(reasoning_effort);
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn with_store(mut self, store: bool) -> Self {
        self.store = Some(store);
        self
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }
}

#[async_trait]
pub trait ChatCompletionModel: Send + Sync {
    async fn chat(
        &self,
        access_token: &str,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ModelError>;
}

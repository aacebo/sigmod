mod criterion;
mod output;

pub use criterion::*;
pub use output::*;

use std::collections::BTreeMap;
use std::fmt::Write;

use async_trait::async_trait;

use crate::{Decision, Evaluate, Meta, ModelId, math};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Scorer {
    /// the model to use.
    pub model: ModelId,

    /// the judges name.
    #[validate(min_length = 2)]
    pub name: String,

    /// Number of top labels to consider for this category
    #[serde(default)]
    #[validate(minimum = 1)]
    pub top_k: Option<usize>,

    /// Weight applied to score when calculating importance.
    #[serde(default = "Scorer::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Scorer::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// the prompt/instructions for the judge.
    #[validate(min_length = 3)]
    pub prompt: String,

    /// criteria the judge evaluates against.
    #[serde(default)]
    #[validate(min_items = 1)]
    pub criteria: Vec<Criterion>,

    /// options set to the LLM when applicable.
    #[serde(default)]
    pub options: Option<BTreeMap<String, serde_json::Value>>,

    /// access token for the LLM provider.
    pub access_token: String,
}

impl Scorer {
    fn default_threshold() -> f32 {
        0.7
    }

    fn default_weight() -> f32 {
        1.0
    }

    fn build_system_prompt(&self) -> String {
        let mut prompt = self.prompt.clone();

        if !self.criteria.is_empty() {
            prompt.push_str("\n\nEvaluate the following criteria:\n");

            for (i, criterion) in self.criteria.iter().enumerate() {
                if let Some(desc) = &criterion.description {
                    let _ = write!(prompt, "{}. {}\n", i + 1, desc);
                } else {
                    let _ = write!(prompt, "{}. Criterion {}\n", i + 1, i + 1);
                }
            }
        }

        prompt.push_str(
            "\nRespond with JSON in this exact format:\n\
             {\n\
             \t\"reasoning\": \"overall reasoning\",\n\
             \t\"criteria\": [\n\
             \t\t{ \"score\": 0.0-1.0, \"reasoning\": \"reasoning for this criterion\" }\n\
             \t]\n\
             }\n",
        );

        prompt
    }
}

#[derive(serde::Deserialize)]
struct JudgeResponse {
    reasoning: Option<String>,
    criteria: Vec<JudgeCriterionResponse>,
}

#[derive(serde::Deserialize)]
struct JudgeCriterionResponse {
    score: f32,
    reasoning: String,
}

#[async_trait]
impl Evaluate for Scorer {
    type Output = Output;

    async fn evaluate(
        &self,
        text: &str,
        client: &ai::client::Client,
    ) -> Result<Self::Output, error::Error> {
        let chat = client
            .as_chat()
            .ok_or_else(|| error::Error::new().with_message("no chat client configured"))?;

        let system_prompt = self.build_system_prompt();

        let mut req = ai::client::chat::ChatCompletionRequest::new(self.model.id.clone())
            .with_messages(vec![
                ai::client::chat::ChatCompletionMessage::System {
                    content: ai::client::chat::Content::Text(system_prompt),
                    name: None,
                },
                ai::client::chat::ChatCompletionMessage::User {
                    content: ai::client::chat::Content::Text(text.to_string()),
                    name: None,
                },
            ])
            .with_response_format(ai::client::chat::ResponseFormat::JsonObject);

        // Apply options if provided
        if let Some(options) = &self.options {
            if let Some(temp) = options.get("temperature").and_then(|v| v.as_f64()) {
                req = req.with_temperature(temp as f32);
            }
            if let Some(top_p) = options.get("top_p").and_then(|v| v.as_f64()) {
                req = req.with_top_p(top_p as f32);
            }
            if let Some(max_tokens) = options
                .get("max_completion_tokens")
                .and_then(|v| v.as_u64())
            {
                req = req.with_max_completion_tokens(max_tokens as u32);
            }
        }

        let res = chat.chat(&self.access_token, req).await?;

        let content = res
            .choices
            .first()
            .and_then(|c| match &c.message {
                ai::client::chat::ChatCompletionMessage::Assistant { content, .. } => {
                    content.clone()
                }
                _ => None,
            })
            .ok_or_else(|| error::Error::new().with_message("no response content from judge"))?;

        let judge_response: JudgeResponse = serde_json::from_str(&content).map_err(|e| {
            error::Error::new()
                .with_message("failed to parse judge response")
                .with_field("error", e)
        })?;

        // Map criteria results
        let mut criterion_results: Vec<CriterionResult> = Vec::new();
        let mut criterion_scores: Vec<(f32, f32)> = Vec::new();

        for (i, criterion) in self.criteria.iter().enumerate() {
            let judge_criterion = judge_response.criteria.get(i);

            let score = judge_criterion.map(|c| c.score).unwrap_or(0.0);
            let reasoning = judge_criterion
                .map(|c| c.reasoning.clone())
                .unwrap_or_default();

            let decision = if score >= criterion.threshold {
                Decision::Accept
            } else {
                Decision::Reject
            };

            criterion_results.push(CriterionResult {
                score,
                decision,
                reasoning,
            });

            criterion_scores.push((score, criterion.weight));
        }

        let score = math::weighted_avg(&criterion_scores);
        let decision = if score >= self.threshold {
            Decision::Accept
        } else {
            Decision::Reject
        };

        let usage = res.usage.map(|u| crate::Usage {
            tokens: u.total_tokens,
            input_tokens: u.prompt_tokens,
            output_tokens: u.completion_tokens,
        });

        Ok(Output {
            meta: Meta {
                request_id: None,
                elapsed_time: None,
                usage,
                other: BTreeMap::new(),
            },
            score,
            decision,
            reasoning: judge_response.reasoning,
            criteria: criterion_results,
        })
    }
}

mod criterion;
mod output;

pub use criterion::*;
pub use output::*;

use std::fmt::Write;

use async_trait::async_trait;

use crate::{Context, Decision, Evaluate, Meta, ModelId, math};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Input {
    /// the model to use.
    pub model: ModelId,

    /// access token for the LLM provider.
    pub access_token: String,

    /// the judges name.
    #[validate(min_length = 2)]
    pub name: String,

    /// Number of top labels to consider for this category
    #[serde(default)]
    #[validate(minimum = 1)]
    pub top_k: Option<usize>,

    /// Weight applied to score when calculating importance.
    #[serde(default = "Input::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Input::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// the prompt/instructions for the judge.
    #[validate(min_length = 3)]
    pub prompt: Option<String>,

    /// criteria the judge evaluates against.
    #[serde(default)]
    #[validate(min_items = 1)]
    pub criteria: Vec<Criterion>,
}

impl Input {
    fn default_threshold() -> f32 {
        0.7
    }

    fn default_weight() -> f32 {
        1.0
    }

    fn system_prompt(&self) -> String {
        let mut prompt = self.prompt.clone().unwrap_or(String::from("You are a judge tasked with scoring the users message text against the following criterion:"));

        for (i, criterion) in self.criteria.iter().enumerate() {
            let _ = write!(prompt, "\n#{} => {}", i + 1, criterion.description);
        }

        prompt
    }
}

#[async_trait]
impl Evaluate for Input {
    type Output = Output;

    async fn evaluate(&self, ctx: &mut Context) -> Result<Self::Output, error::Error> {
        let chat = ctx
            .client(&self.model)
            .ok_or(
                error::Error::new()
                    .with_message("client not found for model provided")
                    .with_field("model", &self.model),
            )?
            .as_chat()
            .ok_or_else(|| error::Error::new().with_message("no chat client configured"))?;

        let system_prompt = self.system_prompt();
        let req = ai::client::chat::ChatCompletionRequest::new(self.model.id.clone())
            .with_messages(vec![
                ai::client::chat::ChatCompletionMessage::System {
                    content: ai::client::chat::Content::Text(system_prompt),
                    name: None,
                },
                ai::client::chat::ChatCompletionMessage::User {
                    content: ai::client::chat::Content::Text(ctx.input().to_string()),
                    name: None,
                },
            ])
            .with_response_format(ai::client::chat::ResponseFormat::JsonSchema {
                json_schema: ai::client::chat::JsonSchema {
                    name: "JudgeResponse".to_string(),
                    schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "reasoning": { "type": "string" },
                            "criteria": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "score": {
                                            "type": "number",
                                            "min": "0",
                                            "max": "1",
                                            "description": "criterion score"
                                        },
                                        "reasoning": {
                                            "type": "string",
                                            "description": "reasoning for this criterion"
                                        }
                                    },
                                    "required": ["score", "reasoning"],
                                    "additionalProperties": false
                                }
                            }
                        },
                        "required": ["criteria"],
                        "additionalProperties": false
                    }),
                    strict: Some(true),
                },
            });

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
            let judge_criterion = judge_response
                .criteria
                .get(i)
                .expect("judge did not score all criterion");

            criterion_results.push(CriterionResult {
                score: judge_criterion.score,
                reasoning: judge_criterion.reasoning.clone(),
                decision: if judge_criterion.score >= criterion.threshold {
                    Decision::Accept
                } else {
                    Decision::Reject
                },
            });

            criterion_scores.push((judge_criterion.score, criterion.weight));
        }

        criterion_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        if let Some(k) = self.top_k {
            criterion_scores.truncate(k);
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
            meta: Meta::new().with(Meta::USAGE, usage)?,
            score,
            decision,
            reasoning: judge_response.reasoning,
            criteria: criterion_results,
        })
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

pub mod classifier;
mod decision;
pub mod judge;
mod meta;

pub use decision::*;
pub use meta::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Scorer {
    Classifier(classifier::Scorer),
    Judge(judge::Scorer),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScorerOutput {
    Classifier(classifier::Output),
    Judge(judge::Output),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct EvalRequest {
    /// request identifier.
    #[serde(default)]
    pub request_id: Option<String>,

    /// the input text being evaluated.
    #[validate(min_length = 3)]
    pub text: String,

    /// the scorers to use.
    #[validate(min_items = 1)]
    pub scorers: Vec<Scorer>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvalResult {
    /// metadata
    #[serde(rename = "$meta")]
    pub meta: Meta,

    /// the score.
    pub score: f32,

    /// the final decision.
    pub decision: Decision,

    /// the individual scorer results
    pub scorers: Vec<ScorerOutput>,
}

pub trait Evaluate {
    fn evaluate(&self, req: &EvalRequest) -> EvalResult;
}

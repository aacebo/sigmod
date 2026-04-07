mod criterion;
mod output;

pub use criterion::*;
pub use output::*;

use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Model {
    Gpt,
    Gpt2,
    GptJ,
    GptNeo,
    XlNet,
    Reformer,
    T5,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Scorer {
    /// the model to use.
    pub model: Model,

    /// the judges name.
    #[validate(min_length = 2)]
    pub name: String,

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
    pub criteria: Vec<Criterion>,

    /// options set to the LLM when applicable.
    #[serde(default)]
    pub options: Option<BTreeMap<String, serde_json::Value>>,
}

impl Scorer {
    fn default_threshold() -> f32 {
        0.75
    }

    fn default_weight() -> f32 {
        1.0
    }
}

use std::collections::BTreeMap;

use crate::judge::{Criterion, Model};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Input {
    /// request identifier.
    #[serde(default)]
    pub request_id: Option<String>,

    /// the model to use.
    pub model: Model,

    /// the judges name.
    #[validate(min_length = 2)]
    pub name: String,

    /// the input text being evaluated.
    #[validate(min_length = 3)]
    pub text: String,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Input::default_threshold")]
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

impl Input {
    fn default_threshold() -> f32 {
        0.75
    }
}

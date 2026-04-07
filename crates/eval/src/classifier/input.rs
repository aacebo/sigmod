use std::collections::BTreeMap;

use crate::classifier::{LabelCategory, Model};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Input {
    /// request identifier.
    #[serde(default)]
    pub request_id: Option<String>,

    /// the model to use.
    pub model: Model,

    /// the input text being evaluated.
    #[validate(min_length = 3)]
    pub text: String,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Input::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// Number of top labels to consider per category (default)
    #[serde(default = "Input::default_top_k")]
    #[validate(minimum = 1)]
    pub top_k: usize,

    /// label categories.
    #[serde(default)]
    #[validate(min_properties = 1)]
    pub categories: BTreeMap<String, LabelCategory>,
}

impl Input {
    fn default_threshold() -> f32 {
        0.75
    }

    fn default_top_k() -> usize {
        2
    }
}

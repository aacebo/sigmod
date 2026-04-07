mod category;
mod label;
mod output;

pub use category::*;
pub use label::*;
pub use output::*;

use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Model {
    Bart,
    Bert,
    Deberta,
    DebertaV2,
    DistilBert,
    Roberta,
    /// custom url.
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Scorer {
    /// the model to use.
    pub model: Model,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Scorer::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// Number of top labels to consider per category (default)
    #[serde(default = "Scorer::default_top_k")]
    #[validate(minimum = 1)]
    pub top_k: usize,

    /// label categories.
    #[serde(default)]
    #[validate(min_properties = 1)]
    pub categories: BTreeMap<String, Category>,
}

impl Scorer {
    fn default_threshold() -> f32 {
        0.75
    }

    fn default_top_k() -> usize {
        2
    }
}

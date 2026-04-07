use std::collections::BTreeMap;

use crate::{
    Decision,
    classifier::{Label, LabelResult},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Category {
    /// Number of top labels to consider for this category
    #[serde(default = "Category::default_top_k")]
    #[validate(minimum = 1)]
    pub top_k: usize,

    /// Weight applied to score when calculating importance.
    #[serde(default = "Category::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Category::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// Labels belonging to this category (keyed by label name)
    #[validate(min_properties = 1)]
    pub labels: BTreeMap<String, Label>,
}

impl Category {
    fn default_top_k() -> usize {
        2
    }

    fn default_weight() -> f32 {
        1.0
    }

    fn default_threshold() -> f32 {
        0.7
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryResult {
    /// the top_k score of the category labels.
    pub score: f32,

    /// decision
    pub decision: Decision,

    /// the individual label results, keyed by label name.
    pub labels: BTreeMap<String, LabelResult>,
}

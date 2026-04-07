use std::collections::BTreeMap;

use crate::classifier::{Label, LabelResult};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct LabelCategory {
    /// Number of top labels to consider for this category
    #[serde(default = "LabelCategory::default_top_k")]
    #[validate(minimum = 1)]
    pub top_k: usize,

    /// Labels belonging to this category (keyed by label name)
    #[validate(min_properties = 1)]
    pub labels: BTreeMap<String, Label>,
}

impl LabelCategory {
    fn default_top_k() -> usize {
        2
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryResult {
    /// the top_k score of the category labels.
    pub score: f32,

    /// average precision of all labels.
    pub precision: f32,

    /// average recall of all labels.
    pub recall: f32,

    /// average f1 of all labels.
    pub f1: f32,

    /// the individual label results, keyed by label name.
    pub labels: BTreeMap<String, LabelResult>,
}

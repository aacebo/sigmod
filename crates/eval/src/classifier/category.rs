use std::collections::BTreeMap;

use crate::{
    ConsensusStrategy, Decision,
    classifier::{Label, LabelResult},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Category {
    /// Consensus strategy for aggregating label scores.
    #[serde(default)]
    pub consensus: ConsensusStrategy,

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
    fn default_weight() -> f32 {
        1.0
    }

    fn default_threshold() -> f32 {
        0.7
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryResult {
    /// the aggregated score of the category labels.
    pub score: f32,

    /// decision
    pub decision: Decision,

    /// the individual label results, keyed by label name.
    pub labels: BTreeMap<String, LabelResult>,
}

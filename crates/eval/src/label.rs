use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Label {
    /// Weight applied to score when calculating importance
    #[serde(default = "Label::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,

    /// Minimum score threshold for this label to be considered
    #[serde(default = "Label::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// Describes the labels meaning
    #[serde(default)]
    #[validate(min_length = 1)]
    pub description: Option<String>,
}

impl Label {
    fn default_weight() -> f32 {
        0.5
    }

    fn default_threshold() -> f32 {
        0.7
    }
}

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

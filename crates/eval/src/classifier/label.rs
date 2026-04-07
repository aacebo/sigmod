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

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct LabelResult {
    /// fraction of predictions that were exactly correct.
    pub score: f32,

    /// of the items the model predicted as positive, how many were actually positive.
    pub precision: f32,

    /// of the items that were actually positive, how many the model found.
    pub recall: f32,

    /// harmonic mean of precision and recall.
    pub f1: f32,
}

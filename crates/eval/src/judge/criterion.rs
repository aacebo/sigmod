use crate::Decision;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Criterion {
    /// Weight applied to score when calculating importance.
    #[serde(default = "Criterion::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,

    /// Minimum score threshold for this criterion to be considered.
    #[serde(default = "Criterion::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// Describes the criterions meaning/hypothesis.
    #[serde(default)]
    #[validate(min_length = 1)]
    pub description: Option<String>,
}

impl Criterion {
    fn default_weight() -> f32 {
        1.0
    }

    fn default_threshold() -> f32 {
        0.7
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CriterionResult {
    /// score for this criterion (0.0 - 1.0).
    pub score: f32,

    /// decision
    pub decision: Decision,

    /// the judge's reasoning for this criterion.
    pub reasoning: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Criterion {
    /// description of what this criterion measures.
    #[validate(min_length = 3)]
    pub text: String,

    /// weight of this criterion in the overall score.
    #[serde(default = "Criterion::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,
}

impl Criterion {
    fn default_weight() -> f32 {
        1.0
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CriterionResult {
    /// score for this criterion (0.0 - 1.0).
    pub score: f32,

    /// the judge's reasoning for this criterion.
    pub reasoning: String,
}

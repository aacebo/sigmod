use crate::{Decision, Meta, judge::CriterionResult};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// metadata
    #[serde(rename = "$meta")]
    pub meta: Meta,

    /// the score.
    pub score: f32,

    /// the final decision.
    pub decision: Decision,

    /// overall reasoning from the judge.
    #[serde(default)]
    pub reasoning: Option<String>,

    /// per-criterion results.
    #[serde(default)]
    pub criteria: Vec<CriterionResult>,
}

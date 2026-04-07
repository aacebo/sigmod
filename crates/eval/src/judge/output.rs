use crate::{Decision, Meta};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// metadata
    #[serde(rename = "$meta")]
    pub meta: Meta,

    /// the score.
    pub score: f32,

    /// the final decision.
    pub decision: Decision,
}

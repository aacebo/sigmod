use std::collections::BTreeMap;

use crate::classifier::CategoryResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// the score.
    pub score: f32,

    /// category results keyed by name.
    pub categories: BTreeMap<String, CategoryResult>,
}

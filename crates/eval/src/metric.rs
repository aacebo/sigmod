use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct LabelCategorySummary {
    /// fraction of predictions that were exactly correct.
    pub accuracy: f32,

    /// average precision of all labels.
    pub precision: f32,

    /// average recall of all labels.
    pub recall: f32,

    /// average f1 of all labels.
    pub f1: f32,

    /// the individual label metrics.
    pub labels: BTreeMap<String, LabelSummary>,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct LabelSummary {
    /// of the items the model predicted as positive, how many were actually positive.
    pub precision: f32,

    /// of the items that were actually positive, how many the model found.
    pub recall: f32,

    /// harmonic mean of precision and recall.
    pub f1: f32,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct SampleCounts {
    /// how many true examples of this label exist
    pub expected: usize,

    /// how many times the model predicted this label
    pub predicted: usize,

    /// how many predictions of this label were correct
    pub correct: usize,
}

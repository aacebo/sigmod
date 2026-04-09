mod category;
mod label;
mod output;

pub use category::*;
pub use label::*;
pub use output::*;

use std::collections::BTreeMap;

use async_trait::async_trait;

use crate::{Decision, Evaluate, Meta, ModelId, math};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Scorer {
    /// the model to use.
    pub model: ModelId,

    /// Number of top labels to consider per category (default)
    #[serde(default = "Scorer::default_top_k")]
    #[validate(minimum = 1)]
    pub top_k: usize,

    /// Weight applied to score when calculating importance.
    #[serde(default = "Scorer::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Scorer::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// label categories.
    #[serde(default)]
    #[validate(min_properties = 1)]
    pub categories: BTreeMap<String, Category>,
}

impl Scorer {
    fn default_threshold() -> f32 {
        0.75
    }

    fn default_top_k() -> usize {
        2
    }

    fn default_weight() -> f32 {
        1.0
    }
}

#[async_trait]
impl Evaluate for Scorer {
    type Output = Output;

    async fn evaluate(
        &self,
        text: &str,
        client: &ai::client::Client,
    ) -> Result<Self::Output, error::Error> {
        let classifier = client
            .as_classifier()
            .ok_or_else(|| error::Error::new().with_message("no classifier client configured"))?;

        // Flatten all labels across categories into a single list for prediction.
        // Track (category_name, label_name) for each to map results back.
        let mut ai_labels: Vec<ai::client::classify::Label> = Vec::new();
        let mut label_index: Vec<(&str, &str)> = Vec::new();

        for (cat_name, category) in &self.categories {
            for (label_name, label) in &category.labels {
                ai_labels.push(ai::client::classify::Label {
                    name: label_name.clone(),
                    description: label.description.clone(),
                });
                label_index.push((cat_name.as_str(), label_name.as_str()));
            }
        }

        let results = classifier.predict(&[text], &ai_labels, 128).await?;
        let sentence_results = results.into_iter().next().unwrap_or_default();

        // Build a score lookup: (category, label) -> raw score
        let mut score_map: BTreeMap<(&str, &str), f64> = BTreeMap::new();

        for result in &sentence_results {
            if let Some(&(cat_name, label_name)) = label_index.get(result.id as usize) {
                score_map.insert((cat_name, label_name), result.score);
            }
        }

        // Compute per-category results
        let mut category_results: BTreeMap<String, CategoryResult> = BTreeMap::new();
        let mut category_scores: Vec<(f32, f32)> = Vec::new();

        for (cat_name, category) in &self.categories {
            let top_k = category.top_k;
            let mut label_results: BTreeMap<String, LabelResult> = BTreeMap::new();
            let mut scored_labels: Vec<(f32, f32)> = Vec::new();

            for (label_name, label) in &category.labels {
                let raw_score = *score_map
                    .get(&(cat_name.as_str(), label_name.as_str()))
                    .unwrap_or(&0.0) as f32;

                let decision = if raw_score >= label.threshold {
                    Decision::Accept
                } else {
                    Decision::Reject
                };

                label_results.insert(
                    label_name.clone(),
                    LabelResult {
                        score: raw_score,
                        decision,
                    },
                );

                scored_labels.push((raw_score, label.weight));
            }

            // Sort by score descending, take top_k
            scored_labels
                .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            scored_labels.truncate(top_k);

            let cat_score = math::weighted_avg(&scored_labels);
            let cat_decision = if cat_score >= category.threshold {
                Decision::Accept
            } else {
                Decision::Reject
            };

            category_results.insert(
                cat_name.clone(),
                CategoryResult {
                    score: cat_score,
                    decision: cat_decision,
                    labels: label_results,
                },
            );

            category_scores.push((cat_score, category.weight));
        }

        let score = weighted_avg(&category_scores);
        let decision = if score >= self.threshold {
            Decision::Accept
        } else {
            Decision::Reject
        };

        Ok(Output {
            meta: Meta {
                request_id: None,
                elapsed_time: None,
                usage: None,
                other: BTreeMap::new(),
            },
            score,
            decision,
            categories: category_results,
        })
    }
}

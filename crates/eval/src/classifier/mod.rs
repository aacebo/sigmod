mod category;
mod label;
mod output;

pub use category::*;
pub use label::*;
pub use output::*;

use std::collections::BTreeMap;

use async_trait::async_trait;

use crate::{ConsensusStrategy, Context, Decision, Evaluate, Meta, ModelId};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Input {
    /// the model to use.
    pub model: ModelId,

    /// Consensus strategy for aggregating category scores.
    #[serde(default)]
    pub consensus: ConsensusStrategy,

    /// Weight applied to score when calculating importance.
    #[serde(default = "Input::default_weight")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub weight: f32,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "Input::default_threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// label categories.
    #[serde(default)]
    #[validate(min_properties = 1)]
    pub categories: BTreeMap<String, Category>,
}

impl Input {
    fn default_threshold() -> f32 {
        0.75
    }

    fn default_weight() -> f32 {
        1.0
    }
}

#[async_trait]
impl Evaluate for Input {
    type Output = Output;

    async fn evaluate(&self, ctx: &mut Context) -> Result<Self::Output, error::Error> {
        let classifier = ctx
            .client(&self.model)
            .ok_or(
                error::Error::new()
                    .with_message("client not found for model provided")
                    .with_field("model", &self.model),
            )?
            .as_classifier()
            .ok_or_else(|| error::Error::new().with_message("no classifier client configured"))?;

        // Flatten all labels across categories into a single list for prediction.
        // Track (category_name, label_name) for each to map results back.
        let started_at = chrono::Utc::now();
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

        // Build a score lookup: (category, label) -> raw score
        let mut score_map: BTreeMap<(&str, &str), f64> = BTreeMap::new();
        let results = classifier.predict(&[ctx.input()], &ai_labels, 128).await?;
        let sentence_results = results.into_iter().next().unwrap_or_default();

        for result in &sentence_results {
            if let Some(&(cat_name, label_name)) = label_index.get(result.id as usize) {
                score_map.insert((cat_name, label_name), result.score);
            }
        }

        // Compute per-category results
        let mut category_results: BTreeMap<String, CategoryResult> = BTreeMap::new();
        let mut category_scores: Vec<(f32, f32)> = Vec::new();
        let mut category_decisions: Vec<Decision> = Vec::new();

        for (cat_name, category) in &self.categories {
            let mut label_results: BTreeMap<String, LabelResult> = BTreeMap::new();
            let mut scored_labels: Vec<(f32, f32)> = Vec::new();
            let mut label_decisions: Vec<Decision> = Vec::new();

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
                label_decisions.push(decision);
            }

            let (cat_score, cat_decision) =
                category
                    .consensus
                    .apply(&scored_labels, &label_decisions, category.threshold);

            category_results.insert(
                cat_name.clone(),
                CategoryResult {
                    score: cat_score,
                    decision: cat_decision,
                    labels: label_results,
                },
            );

            category_scores.push((cat_score, category.weight));
            category_decisions.push(cat_decision);
        }

        let (score, decision) =
            self.consensus
                .apply(&category_scores, &category_decisions, self.threshold);
        let elapse = chrono::Utc::now() - started_at;

        Ok(Output {
            meta: Meta::new().with(Meta::ELAPSED_MS, format!("{}ms", elapse.num_milliseconds()))?,
            score,
            decision,
            categories: category_results,
        })
    }
}

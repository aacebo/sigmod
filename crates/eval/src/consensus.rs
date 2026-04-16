use crate::{Decision, math};

fn default_k() -> usize {
    2
}

/// Consensus strategy for aggregating multiple scored items into a single
/// score and decision.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConsensusStrategy {
    /// Top-K Filtering with Weighted Aggregation.
    ///
    /// Selects the `k` most important items by importance (score × weight),
    /// then computes their weighted average. Decision = score >= parent threshold.
    ///
    /// Use when you have many sub-items but only the most impactful ones should
    /// drive the final score (e.g., focus on the 3 most relevant labels in a
    /// category with 20+ labels).
    ///
    /// Related: Top-K selection in information retrieval and recommendation systems.
    /// See: <https://en.wikipedia.org/wiki/Top-k_retrieval>
    TopK {
        #[serde(default = "default_k")]
        k: usize,
    },

    /// Soft Voting (Weighted Average Aggregation).
    ///
    /// Computes weighted average of ALL items. Decision = score >= parent threshold.
    /// No filtering — all items contribute proportionally to their weight.
    ///
    /// Use when every item matters and should contribute to the final score
    /// proportionally (e.g., all criteria in a rubric carry meaning).
    ///
    /// Known as "soft voting" in ensemble learning.
    /// See: <https://scikit-learn.org/stable/modules/ensemble.html#voting-classifier>
    Average,

    /// Majority Voting (Hard Vote).
    ///
    /// Computes weighted average of ALL items. Decision = >50% of items (by count)
    /// individually pass their thresholds.
    ///
    /// Use when you want a democratic decision — the group passes if most members
    /// agree (e.g., multiple judges scoring an essay, accept if most approve).
    ///
    /// Known as "plurality voting" or "hard voting" in ensemble methods.
    /// See: <https://scikit-learn.org/stable/modules/ensemble.html#voting-classifier>
    Majority,

    /// Disjunctive Consensus (Existential Quantification).
    ///
    /// Computes weighted average of ALL items. Decision = any item passes its threshold.
    /// Optimistic strategy — a single positive signal is sufficient.
    ///
    /// Use when any single positive signal is sufficient to accept (e.g., content
    /// matches at least one of several acceptable categories).
    ///
    /// Known as "logical OR" or "existential consensus" in multi-agent systems.
    /// See: <https://arxiv.org/abs/2310.20151>
    AtLeastOne,

    /// Unanimous Consensus (Universal Quantification).
    ///
    /// Computes weighted average of ALL items. Decision = ALL items pass their thresholds.
    /// Conservative/veto-based — a single rejection rejects the whole group.
    ///
    /// Use for safety-critical evaluations where every criterion must pass — a single
    /// failure vetoes the result (e.g., content moderation, compliance checks).
    ///
    /// Known as "unanimity rule" or "veto voting" in social choice theory.
    /// See: <https://en.wikipedia.org/wiki/Unanimity>
    Unanimous,
}

impl Default for ConsensusStrategy {
    fn default() -> Self {
        Self::TopK { k: default_k() }
    }
}

impl ConsensusStrategy {
    /// Apply the consensus strategy to a set of scored items.
    ///
    /// - `items`: `(score, weight)` pairs for each item.
    /// - `decisions`: per-item decisions (from individual threshold comparisons).
    /// - `threshold`: the parent threshold for strategies that use threshold-based decisions.
    ///
    /// Returns `(aggregated_score, final_decision)`.
    pub fn apply(
        &self,
        items: &[(f32, f32)],
        decisions: &[Decision],
        threshold: f32,
    ) -> (f32, Decision) {
        match self {
            ConsensusStrategy::TopK { k } => {
                let mut sorted = items.to_vec();

                sorted.sort_by(|a, b| {
                    let imp_b = b.0 * b.1;
                    let imp_a = a.0 * a.1;
                    imp_b
                        .partial_cmp(&imp_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                sorted.truncate(*k);

                let score = math::weighted_avg(&sorted);
                let decision = if score >= threshold {
                    Decision::Accept
                } else {
                    Decision::Reject
                };

                (score, decision)
            }
            ConsensusStrategy::Average => {
                let score = math::weighted_avg(items);
                let decision = if score >= threshold {
                    Decision::Accept
                } else {
                    Decision::Reject
                };

                (score, decision)
            }
            ConsensusStrategy::Majority => {
                let score = math::weighted_avg(items);
                let accept_count = decisions.iter().filter(|d| **d == Decision::Accept).count();
                let decision = if accept_count > decisions.len() / 2 {
                    Decision::Accept
                } else {
                    Decision::Reject
                };

                (score, decision)
            }
            ConsensusStrategy::AtLeastOne => {
                let score = items
                    .iter()
                    .map(|(s, _)| *s)
                    .fold(0.0f32, f32::max);
                let decision = if decisions.iter().any(|d| *d == Decision::Accept) {
                    Decision::Accept
                } else {
                    Decision::Reject
                };

                (score, decision)
            }
            ConsensusStrategy::Unanimous => {
                let score = math::weighted_avg(items);
                let decision = if decisions.iter().all(|d| *d == Decision::Accept) {
                    Decision::Accept
                } else {
                    Decision::Reject
                };

                (score, decision)
            }
        }
    }
}

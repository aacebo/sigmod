use crate::{Decision, EvalId, EvalRequest, EvalResult, Evaluate, Meta, ScorerOutput, math};

pub struct Context<'a> {
    id: EvalId,
    meta: Meta,
    input: &'a str,
    registry: &'a ai::model::ModelRegistry,
}

impl<'a> Context<'a> {
    pub fn id(&self) -> EvalId {
        self.id
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    pub fn meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn client(&self, model: &ai::model::ModelId) -> Option<&ai::client::Client> {
        self.registry.get(model)
    }
}

#[derive(Clone)]
pub struct Runner {
    registry: ai::model::ModelRegistry,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            registry: ai::model::ModelRegistry::new(),
        }
    }

    pub fn with(mut self, model: ai::model::ModelId, client: ai::client::Client) -> Self {
        self.registry.register(model, client);
        self
    }

    pub async fn evaluate(&self, req: &EvalRequest) -> EvalResult {
        let id = EvalId::new();
        let mut scorers: Vec<ScorerOutput> = vec![];
        let mut ctx = Context {
            id,
            meta: Meta::new(),
            input: &req.input,
            registry: &self.registry,
        };

        for input in &req.scorers {
            scorers.push(match input.evaluate(&mut ctx).await {
                Err(err) => err.into(),
                Ok(v) => v.into(),
            });
        }

        let scores: Vec<(f32, f32)> = req
            .scorers
            .iter()
            .zip(scorers.iter())
            .filter_map(|(input, output)| match output {
                ScorerOutput::Error(_) => None,
                ScorerOutput::Classifier(o) => Some((o.score, input.weight())),
                ScorerOutput::Judge(o) => Some((o.score, input.weight())),
            })
            .collect();

        let score = math::weighted_avg(&scores);
        let decision = if scorers.iter().any(|v| v.decision() == Decision::Reject) {
            Decision::Reject
        } else {
            Decision::Accept
        };

        EvalResult {
            meta: ctx.meta,
            id,
            score,
            decision,
            scorers,
        }
    }
}

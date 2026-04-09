use crate::{Decision, EvalId, EvalRequest, EvalResult, Evaluate, Meta, ScorerOutput};

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

        EvalResult {
            meta: ctx.meta,
            id,
            score: 0.0,
            decision: Decision::Accept,
            scorers,
        }
    }
}

use crate::Meta;

#[derive(Default)]
pub struct Context {
    meta: Meta,
    input: String,
    registry: ai::model::ModelRegistry,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
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

pub struct ContextBuilder {
    meta: Meta,
    input: String,
    registry: ai::model::ModelRegistry,
}

impl ContextBuilder {
    pub fn new(input: String) -> Self {
        Self {
            meta: Meta::default(),
            input,
            registry: ai::model::ModelRegistry::default(),
        }
    }

    pub fn registry(mut self, registry: ai::model::ModelRegistry) -> Self {
        self.registry = registry;
        self
    }

    pub fn client(mut self, model: ai::model::ModelId, client: ai::client::Client) -> Self {
        self.registry.register(model, client);
        self
    }

    pub fn meta(mut self, value: Meta) -> Self {
        self.meta = value;
        self
    }

    pub fn build(self) -> Context {
        Context {
            meta: self.meta,
            input: self.input,
            registry: self.registry,
        }
    }
}

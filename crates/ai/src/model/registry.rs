use std::{collections::BTreeMap, sync::Arc};

use crate::client::{Client, chat, classify};
use crate::model::ModelId;

#[derive(Default, Clone)]
pub struct ModelRegistry {
    items: BTreeMap<ModelId, Client>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn register_chat(&mut self, id: ModelId, model: impl chat::ChatCompletionClient + 'static) {
        self.items.insert(
            id,
            ModelEntry {
                chat: Some(Arc::new(model)),
                classifier: None,
            },
        );
    }

    pub fn register_classifier(
        &mut self,
        id: ModelId,
        model: impl classify::ClassificationClient + 'static,
    ) {
        todo!()
    }

    pub fn chat(&self, id: &ModelId) -> Option<&dyn chat::ChatCompletionClient> {
        self.items.get(id)?.chat.as_deref()
    }

    pub fn classifier(&self, id: &ModelId) -> Option<&dyn classify::ClassificationClient> {
        self.items.get(id)?.classifier.as_deref()
    }
}

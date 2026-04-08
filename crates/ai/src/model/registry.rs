use std::{collections::BTreeMap, sync::Arc};

use crate::model::{ChatCompletionClient, ClassificationClient, ModelId};

#[derive(Default)]
pub struct ModelRegistry {
    items: BTreeMap<ModelId, ModelEntry>,
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

    pub fn register_chat(&mut self, id: ModelId, model: impl ChatCompletionClient + 'static) {
        self.items.insert(
            id,
            ModelEntry {
                chat: Some(Arc::new(model)),
                classifier: None,
            },
        );
    }

    pub fn register_classifier(&mut self, id: ModelId, model: impl ClassificationClient + 'static) {
        self.items.insert(
            id,
            ModelEntry {
                chat: None,
                classifier: Some(Arc::new(model)),
            },
        );
    }

    pub fn chat(&self, id: &ModelId) -> Option<&dyn ChatCompletionClient> {
        self.items.get(id)?.chat.as_deref()
    }

    pub fn classifier(&self, id: &ModelId) -> Option<&dyn ClassificationClient> {
        self.items.get(id)?.classifier.as_deref()
    }
}

struct ModelEntry {
    chat: Option<Arc<dyn ChatCompletionClient>>,
    classifier: Option<Arc<dyn ClassificationClient>>,
}

use std::{collections::BTreeMap, sync::Arc};

use crate::model::{ChatCompletionModel, ClassificationModel, ModelId};

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

    pub fn register_chat(&mut self, model: impl ChatCompletionModel + 'static) {
        self.items.insert(
            model.id(),
            ModelEntry {
                chat: Some(Arc::new(model)),
                classifier: None,
            },
        );
    }

    pub fn register_classifier(&mut self, model: impl ClassificationModel + 'static) {
        self.items.insert(
            model.id(),
            ModelEntry {
                chat: None,
                classifier: Some(Arc::new(model)),
            },
        );
    }

    pub fn chat(&self, id: &ModelId) -> Option<&dyn ChatCompletionModel> {
        self.items.get(id)?.chat.as_deref()
    }

    pub fn classifier(&self, id: &ModelId) -> Option<&dyn ClassificationModel> {
        self.items.get(id)?.classifier.as_deref()
    }
}

struct ModelEntry {
    chat: Option<Arc<dyn ChatCompletionModel>>,
    classifier: Option<Arc<dyn ClassificationModel>>,
}

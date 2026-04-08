use std::collections::BTreeMap;

use crate::client::Client;
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

    pub fn get(&self, id: &ModelId) -> Option<&Client> {
        self.items.get(id)
    }

    pub fn get_mut(&mut self, id: &ModelId) -> Option<&mut Client> {
        self.items.get_mut(id)
    }

    pub fn register(&mut self, id: ModelId, client: Client) {
        self.items.insert(id, client);
    }
}

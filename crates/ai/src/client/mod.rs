pub mod chat;
pub mod classify;

use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait RequestMiddleware {
    type Err;

    async fn execute<Next: FnOnce(reqwest::Request) -> Result<reqwest::Response, Self::Err>>(
        &self,
        req: reqwest::Request,
        next: Next,
    );
}

#[derive(Default, Clone)]
pub struct Client {
    chat: Option<Arc<dyn chat::ChatCompletionClient>>,
    classifier: Option<Arc<dyn classify::ClassificationClient>>,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_chat(mut self, client: impl chat::ChatCompletionClient + 'static) -> Self {
        self.chat = Some(Arc::new(client));
        self
    }

    pub fn with_classifier(
        mut self,
        client: impl classify::ClassificationClient + 'static,
    ) -> Self {
        self.classifier = Some(Arc::new(client));
        self
    }

    pub fn set_chat(&mut self, client: impl chat::ChatCompletionClient + 'static) {
        self.chat = Some(Arc::new(client));
    }

    pub fn set_classifier(&mut self, client: impl classify::ClassificationClient + 'static) {
        self.classifier = Some(Arc::new(client));
    }

    pub fn as_chat(&self) -> Option<&dyn chat::ChatCompletionClient> {
        self.chat.as_deref()
    }

    pub fn as_classifier(&self) -> Option<&dyn classify::ClassificationClient> {
        self.classifier.as_deref()
    }
}

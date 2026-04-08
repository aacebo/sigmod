use async_trait::async_trait;

use crate::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Label {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LabelResult {
    pub id: i64,
    pub text: String,
    pub score: f64,
    pub sentence: usize,
}

#[async_trait]
pub trait ClassificationClient: Send + Sync {
    async fn predict(
        &self,
        inputs: &[&str],
        labels: &[Label],
        max_length: usize,
    ) -> Result<Vec<Vec<LabelResult>>, Error>;
}

use async_trait::async_trait;

use crate::model::ModelError;

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
pub trait ClassificationModel: Send + Sync {
    async fn predict(
        &self,
        inputs: &[&str],
        labels: &[Label],
        max_length: usize,
    ) -> Result<Vec<Vec<LabelResult>>, ModelError>;
}

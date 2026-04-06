use std::collections::BTreeMap;

use crate::build::ScorerBuilder;
use crate::entity::ScorerType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Scorer {
    pub id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub last_run_id: Option<uuid::Uuid>,
    #[sqlx(rename = "type")]
    pub ty: ScorerType,
    pub name: Option<String>,
    pub description: Option<String>,
    pub options: Option<sqlx::types::Json<BTreeMap<String, serde_json::Value>>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Scorer {
    pub fn builder(project_id: uuid::Uuid, ty: ScorerType) -> ScorerBuilder {
        ScorerBuilder::new(project_id, ty)
    }
}

use std::collections::BTreeMap;

use crate::build::RunBuilder;
use crate::entity::RunStatus;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Run {
    pub id: uuid::Uuid,
    pub scorer_id: uuid::Uuid,
    pub status: RunStatus,
    pub input: String,
    pub output: Option<sqlx::types::Json<BTreeMap<String, serde_json::Value>>>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Run {
    pub fn builder(scorer_id: uuid::Uuid, input: impl Into<String>) -> RunBuilder {
        RunBuilder::new(scorer_id, input)
    }
}

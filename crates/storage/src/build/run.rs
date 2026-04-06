use std::collections::BTreeMap;

use crate::entity::{Run, RunStatus};

#[derive(Debug, Clone)]
pub struct RunBuilder {
    scorer_id: uuid::Uuid,
    status: RunStatus,
    input: String,
    output: Option<sqlx::types::Json<BTreeMap<String, serde_json::Value>>>,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    ended_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl RunBuilder {
    pub fn new(scorer_id: uuid::Uuid, input: impl Into<String>) -> Self {
        Self {
            scorer_id,
            status: RunStatus::Pending,
            input: input.into(),
            output: None,
            started_at: None,
            ended_at: None,
        }
    }

    pub fn status(mut self, status: RunStatus) -> Self {
        self.status = status;
        self
    }

    pub fn output(mut self, output: BTreeMap<String, serde_json::Value>) -> Self {
        self.output = Some(sqlx::types::Json(output));
        self
    }

    pub fn started_at(mut self, started_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.started_at = Some(started_at);
        self
    }

    pub fn ended_at(mut self, ended_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.ended_at = Some(ended_at);
        self
    }

    pub fn build(self) -> Run {
        let now = chrono::Utc::now();
        Run {
            id: uuid::Uuid::new_v4(),
            scorer_id: self.scorer_id,
            status: self.status,
            input: self.input,
            output: self.output,
            started_at: self.started_at,
            ended_at: self.ended_at,
            created_at: now,
            updated_at: now,
        }
    }
}

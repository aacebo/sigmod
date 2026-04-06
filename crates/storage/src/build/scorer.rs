use std::collections::BTreeMap;

use crate::entity::{Scorer, ScorerType};

#[derive(Debug, Clone)]
pub struct ScorerBuilder {
    project_id: uuid::Uuid,
    last_run_id: Option<uuid::Uuid>,
    ty: ScorerType,
    name: Option<String>,
    description: Option<String>,
    options: Option<sqlx::types::Json<BTreeMap<String, serde_json::Value>>>,
}

impl ScorerBuilder {
    pub fn new(project_id: uuid::Uuid, ty: ScorerType) -> Self {
        Self {
            project_id,
            last_run_id: None,
            ty,
            name: None,
            description: None,
            options: None,
        }
    }

    pub fn last_run_id(mut self, last_run_id: uuid::Uuid) -> Self {
        self.last_run_id = Some(last_run_id);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn options(mut self, options: BTreeMap<String, serde_json::Value>) -> Self {
        self.options = Some(sqlx::types::Json(options));
        self
    }

    pub fn build(self) -> Scorer {
        let now = chrono::Utc::now();
        Scorer {
            id: uuid::Uuid::new_v4(),
            project_id: self.project_id,
            last_run_id: self.last_run_id,
            ty: self.ty,
            name: self.name,
            description: self.description,
            options: self.options,
            created_at: now,
            updated_at: now,
        }
    }
}

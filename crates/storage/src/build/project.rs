use crate::entity::Project;

#[derive(Debug, Clone)]
pub struct ProjectBuilder {
    workspace_id: uuid::Uuid,
    name: String,
    secret: String,
    description: Option<String>,
}

impl ProjectBuilder {
    pub fn new(
        workspace_id: uuid::Uuid,
        name: impl Into<String>,
        secret: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id,
            name: name.into(),
            secret: secret.into(),
            description: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> Project {
        let now = chrono::Utc::now();
        Project {
            id: uuid::Uuid::new_v4(),
            workspace_id: self.workspace_id,
            name: self.name,
            secret: self.secret,
            description: self.description,
            created_at: now,
            updated_at: now,
        }
    }
}

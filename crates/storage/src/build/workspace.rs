use crate::entity::Workspace;

#[derive(Debug, Clone)]
pub struct WorkspaceBuilder {
    name: String,
    secret: String,
}

impl WorkspaceBuilder {
    pub fn new(name: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            secret: secret.into(),
        }
    }

    pub fn build(self) -> Workspace {
        let now = chrono::Utc::now();
        Workspace {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            secret: self.secret,
            created_at: now,
            updated_at: now,
        }
    }
}

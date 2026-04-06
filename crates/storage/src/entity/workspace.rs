use crate::build::WorkspaceBuilder;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Workspace {
    pub id: uuid::Uuid,
    pub name: String,
    pub secret: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Workspace {
    pub fn builder(name: impl Into<String>, secret: impl Into<String>) -> WorkspaceBuilder {
        WorkspaceBuilder::new(name, secret)
    }
}

use crate::build::ProjectBuilder;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub name: String,
    pub secret: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Project {
    pub fn builder(
        workspace_id: uuid::Uuid,
        name: impl Into<String>,
        secret: impl Into<String>,
    ) -> ProjectBuilder {
        ProjectBuilder::new(workspace_id, name, secret)
    }
}

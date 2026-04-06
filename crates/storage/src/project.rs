use sqlx::PgPool;

use crate::entity::Project;

pub struct ProjectStorage<'a> {
    pool: &'a PgPool,
}

impl<'a> ProjectStorage<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await
    }

    pub async fn get_by_workspace(
        &self,
        workspace_id: uuid::Uuid,
    ) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE workspace_id = $1")
            .bind(workspace_id)
            .fetch_all(self.pool)
            .await
    }

    pub async fn create(&self, project: &Project) -> Result<Project, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            r#"
            INSERT INTO projects (id, workspace_id, name, secret, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(project.id)
        .bind(project.workspace_id)
        .bind(&project.name)
        .bind(&project.secret)
        .bind(&project.description)
        .fetch_one(self.pool)
        .await
    }

    pub async fn update(&self, project: &Project) -> Result<Option<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            r#"
            UPDATE projects
            SET name = $2, secret = $3, description = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.secret)
        .bind(&project.description)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

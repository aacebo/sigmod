use sqlx::PgPool;

use crate::entity::Workspace;

pub struct WorkspaceStorage<'a> {
    pool: &'a PgPool,
}

impl<'a> WorkspaceStorage<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<Workspace>, sqlx::Error> {
        sqlx::query_as::<_, Workspace>("SELECT * FROM workspaces WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await
    }

    pub async fn create(&self, workspace: &Workspace) -> Result<Workspace, sqlx::Error> {
        sqlx::query_as::<_, Workspace>(
            r#"
            INSERT INTO workspaces (id, name, secret, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(workspace.id)
        .bind(&workspace.name)
        .bind(&workspace.secret)
        .fetch_one(self.pool)
        .await
    }

    pub async fn update(&self, workspace: &Workspace) -> Result<Option<Workspace>, sqlx::Error> {
        sqlx::query_as::<_, Workspace>(
            r#"
            UPDATE workspaces
            SET name = $2, secret = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(workspace.id)
        .bind(&workspace.name)
        .bind(&workspace.secret)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM workspaces WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

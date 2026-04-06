use sqlx::PgPool;

use crate::entity::Scorer;

pub struct ScorerStorage<'a> {
    pool: &'a PgPool,
}

impl<'a> ScorerStorage<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<Scorer>, sqlx::Error> {
        sqlx::query_as::<_, Scorer>("SELECT * FROM scorers WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await
    }

    pub async fn get_by_project(&self, project_id: uuid::Uuid) -> Result<Vec<Scorer>, sqlx::Error> {
        sqlx::query_as::<_, Scorer>("SELECT * FROM scorers WHERE project_id = $1")
            .bind(project_id)
            .fetch_all(self.pool)
            .await
    }

    pub async fn create(&self, scorer: &Scorer) -> Result<Scorer, sqlx::Error> {
        sqlx::query_as::<_, Scorer>(
            r#"
            INSERT INTO scorers (id, project_id, last_run_id, type, name, description, options, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(scorer.id)
        .bind(scorer.project_id)
        .bind(scorer.last_run_id)
        .bind(scorer.ty)
        .bind(&scorer.name)
        .bind(&scorer.description)
        .bind(&scorer.options)
        .fetch_one(self.pool)
        .await
    }

    pub async fn update(&self, scorer: &Scorer) -> Result<Option<Scorer>, sqlx::Error> {
        sqlx::query_as::<_, Scorer>(
            r#"
            UPDATE scorers
            SET last_run_id = $2, type = $3, name = $4, description = $5, options = $6, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(scorer.id)
        .bind(scorer.last_run_id)
        .bind(scorer.ty)
        .bind(&scorer.name)
        .bind(&scorer.description)
        .bind(&scorer.options)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM scorers WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

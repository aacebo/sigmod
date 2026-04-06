use sqlx::PgPool;

use crate::entity::Run;

pub struct RunStorage<'a> {
    pool: &'a PgPool,
}

impl<'a> RunStorage<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<Run>, sqlx::Error> {
        sqlx::query_as::<_, Run>("SELECT * FROM runs WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await
    }

    pub async fn get_by_scorer(&self, scorer_id: uuid::Uuid) -> Result<Vec<Run>, sqlx::Error> {
        sqlx::query_as::<_, Run>("SELECT * FROM runs WHERE scorer_id = $1")
            .bind(scorer_id)
            .fetch_all(self.pool)
            .await
    }

    pub async fn create(&self, run: &Run) -> Result<Run, sqlx::Error> {
        sqlx::query_as::<_, Run>(
            r#"
            INSERT INTO runs (id, scorer_id, status, input, output, started_at, ended_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(run.id)
        .bind(run.scorer_id)
        .bind(run.status)
        .bind(&run.input)
        .bind(&run.output)
        .bind(run.started_at)
        .bind(run.ended_at)
        .fetch_one(self.pool)
        .await
    }

    pub async fn update(&self, run: &Run) -> Result<Option<Run>, sqlx::Error> {
        sqlx::query_as::<_, Run>(
            r#"
            UPDATE runs
            SET status = $2, output = $3, started_at = $4, ended_at = $5, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(run.id)
        .bind(run.status)
        .bind(&run.output)
        .bind(run.started_at)
        .bind(run.ended_at)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM runs WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

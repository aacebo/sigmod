use sqlx::PgPool;

pub mod build;
pub mod entity;

mod project_storage;
mod run_storage;
mod scorer_storage;
mod workspace_storage;

pub use project_storage::*;
pub use run_storage::*;
pub use scorer_storage::*;
pub use workspace_storage::*;

pub struct Storage<'a> {
    pub workspaces: WorkspaceStorage<'a>,
    pub projects: ProjectStorage<'a>,
    pub scorers: ScorerStorage<'a>,
    pub runs: RunStorage<'a>,
}

impl<'a> Storage<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self {
            workspaces: WorkspaceStorage::new(pool),
            projects: ProjectStorage::new(pool),
            scorers: ScorerStorage::new(pool),
            runs: RunStorage::new(pool),
        }
    }
}

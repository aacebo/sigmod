use sqlx::PgPool;

pub mod build;
pub mod entity;

mod project;
mod run;
mod scorer;
mod workspace;

pub use project::*;
pub use run::*;
pub use scorer::*;
pub use workspace::*;

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

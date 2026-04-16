use chrono::{DateTime, Utc};
use sqlx::PgPool;

use amqp::Socket;
use storage::Storage;

#[derive(Clone)]
pub struct Context {
    pool: PgPool,
    amqp: Socket,
    runner: eval::Runner,
    start_time: DateTime<Utc>,
}

impl Context {
    pub fn new(pool: PgPool, amqp: Socket, runner: eval::Runner) -> Self {
        Self {
            pool,
            amqp,
            runner,
            start_time: Utc::now(),
        }
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn storage(&self) -> Storage<'_> {
        Storage::new(&self.pool)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn amqp(&self) -> &Socket {
        &self.amqp
    }

    pub fn runner(&self) -> &eval::Runner {
        &self.runner
    }
}

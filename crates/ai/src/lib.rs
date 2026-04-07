pub mod model;
pub mod openai;

use async_trait::async_trait;

#[async_trait]
pub trait RequestMiddleware {
    type Err;

    async fn execute<Next: FnOnce(reqwest::Request) -> Result<reqwest::Response, Self::Err>>(
        &self,
        req: reqwest::Request,
        next: Next,
    );
}

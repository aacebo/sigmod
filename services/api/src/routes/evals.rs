use actix_web::{HttpResponse, post, web};

use crate::RequestContext;

#[post("/evals")]
pub async fn create(ctx: RequestContext, payload: web::Json<eval::EvalRequest>) -> HttpResponse {
    let req = payload.into_inner();
    let result = ctx.runner().evaluate(&req).await;
    HttpResponse::Ok().json(result)
}

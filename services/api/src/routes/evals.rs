use actix_web::{HttpResponse, post, web};

use crate::RequestContext;

#[post("/evals")]
pub async fn create(ctx: RequestContext, payload: web::Json<eval::EvalRequest>) -> HttpResponse {
    let _ctx = ctx.context();
    let _req = payload.into_inner();

    HttpResponse::Ok().finish()
}

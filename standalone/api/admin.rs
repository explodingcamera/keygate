use crate::KG;
use actix_web::{get, web, Responder, Scope};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(super::admin::test))]
pub struct AdminApiDoc;

pub fn get() -> Scope {
    let identity = web::scope("/admin").service(test);
    web::scope("/v1").service(identity)
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, body = String)
    )
)]
#[get("/test")]
async fn test(ks: KG) -> impl Responder {
    ks.identity.get("something").unwrap().unwrap().username;
    "sdf"
}

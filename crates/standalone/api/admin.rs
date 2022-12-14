use actix_web::{web, Scope};
use utoipa::OpenApi;

mod schema;

#[derive(OpenApi)]
#[openapi(paths())]
pub struct AdminApiDoc;

pub fn service(scope: &str) -> Scope {
    let identity = web::scope("/admin");
    web::scope(scope).service(identity)
}

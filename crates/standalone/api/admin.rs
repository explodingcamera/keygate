use actix_web::{web, Scope};
use utoipa::OpenApi;

mod schema;

#[derive(OpenApi)]
#[openapi(paths())]
pub struct AdminApiDoc;

pub fn get() -> Scope {
    let identity = web::scope("/admin");
    web::scope("/v1").service(identity)
}

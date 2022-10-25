use actix_web::{web, Scope};

pub fn get() -> Scope {
    let identity = web::scope("/admin");
    web::scope("/v1").service(identity)
}

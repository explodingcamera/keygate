use crate::KS;
use actix_web::{get, web, Responder, Scope};

pub fn get() -> Scope {
    let identity = web::scope("/admin");
    web::scope("/v1").service(identity)
}

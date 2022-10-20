use crate::KS;
use actix_web::{get, web, Responder, Scope};

pub fn get() -> Scope {
    let identity = web::scope("/admin").service(test2).service(test);
    web::scope("/v1").service(identity)
}

#[get("/test2")]
async fn test2() -> impl Responder {
    "Hello!"
}

#[get("/test")]
async fn test(ks: KS) -> impl Responder {
    "Hello"
}

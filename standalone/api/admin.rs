use actix_web::{get, web, Responder, Scope};

use crate::KS;

pub fn get() -> Scope {
    web::scope("/identity").service(test).service(test2)
}

#[get("/test2")]
async fn test2() -> impl Responder {
    "Hello!"
}

#[get("/test")]
async fn test(ks: KS) -> impl Responder {
    "Hello"
}

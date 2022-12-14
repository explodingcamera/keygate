use actix_web::{dev::HttpServiceFactory, web};

pub fn service(scope: &str) -> impl HttpServiceFactory {
    web::scope(scope)
}

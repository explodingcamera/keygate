use actix_cors::Cors;
use actix_web::{
    dev::HttpServiceFactory,
    http::{self, Uri},
    middleware::DefaultHeaders,
    web::{self},
};

use keygate_core::config::Environment;
use utoipa::OpenApi;

mod identity;
mod process_login;
mod process_signup;
mod schema;
mod session;

use crate::{errors::KeygateErrorResponse, KG};

pub fn service(scope: &str, kg: KG) -> impl HttpServiceFactory {
    let session = session::service("/session");
    let identity = identity::service("/identity");
    let process_login = process_login::service("/process/login");
    let process_signup = process_signup::service("/process/signup");
    // let process_recovery = process_recovery::service("/process/recovery");

    let environment = { kg.config.read().unwrap().environment.clone() };

    let mut default_headers = DefaultHeaders::new()
        .add(("x-xss-protection", "1; mode=block"))
        .add(("x-content-type-options", "nosniff"))
        .add(("x-frame-options", "DENY"))
        .add(("referrer-policy", "no-referrer"))
        .add(("Content-Security-Policy", "default-src 'none'"));

    if environment == Environment::Production {
        default_headers = default_headers.add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"));
    }

    let cors = match environment {
        Environment::Production => Cors::default(),
        Environment::Development => Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                let Ok(Ok(uri)) = origin.to_str().map(|s| s.parse::<Uri>()) else {
                    return false;
                };

                if let Some(host) = uri.host() {
                    return host == "localhost" || host.ends_with(".localhost") || host == "127.0.0.1";
                }

                false
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600),
    };

    web::scope(scope)
        .wrap(cors)
        .wrap(default_headers)
        .service(session)
        .service(identity)
        .service(process_login)
        .service(process_signup)
        .route("/ping", web::get().to(pong))
    // .service(process_recovery)
}

async fn pong() -> &'static str {
    "pong"
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // /api/v1/session
        session::refresh,
        // /api/v1/process/login
        process_login::create_login_process
    ),
    components(schemas(
        // /api/v1/session
        schema::RefreshResponse,
        // /api/v1/process/login
        schema::LoginProcessRequest,
        schema::LoginProcessResponse,
        process_login::LoginProcessStep,

        KeygateErrorResponse
    ))
)]
pub struct PublicApiDoc;

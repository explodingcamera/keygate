use actix_web::{
    web::{self},
    Scope,
};

use utoipa::OpenApi;

mod identity;
mod process_login;
mod process_signup;
mod schema;
mod session;

use crate::errors::KeygateErrorResponse;

pub fn get() -> Scope {
    let session = web::scope("/session").service(session::refresh);
    let identity = web::scope("/identity").service(session::refresh);
    let process_login = web::scope("/process/login").service(session::refresh);
    let process_signup = web::scope("/process/signup").service(session::refresh);
    let process_recovery = web::scope("/process/recovery").service(session::refresh);

    web::scope("/")
        .service(session)
        .service(identity)
        .service(process_login)
        .service(process_signup)
        .service(process_recovery)
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

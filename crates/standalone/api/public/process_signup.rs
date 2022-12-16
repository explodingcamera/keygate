use actix_web::{
    dev::HttpServiceFactory,
    post, put,
    web::{self, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    utils::{create_refresh_token_cookie, get_refresh_token_cookie_config, response, HttpResult, JsonResult},
    KG,
};

pub fn service(scope: &str) -> impl HttpServiceFactory {
    web::scope(scope).service(create_signup_process)
}

#[derive(Serialize, ToSchema)]
pub enum SignupProcessStep {
    // user needs to enter a password
    RequirePassword,

    // email has to be verified before login
    RequireEmailVerification,

    // user has to be approved before login
    RequireApproval,
}

#[derive(Deserialize, ToSchema)]
pub struct SignupProcessRequest {
    username: Option<String>,
    email: Option<String>,
    device_id: String,
}

#[derive(Serialize, ToSchema)]
pub struct SignupProcessResponse {
    expires_at: i64,
    process_id: String,
    next_step: SignupProcessStep,
    access_token: Option<String>,
}

#[utoipa::path(
    tag = "Signup Process",
    context_path = "/api/v1/process/signup",
    request_body = SignupProcessRequest,
    responses(
        (status = 200, body = SignupProcessResponse),
        (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
    ),
  )]
#[put("")]
async fn create_signup_process(req: Json<SignupProcessRequest>, kg: KG) -> JsonResult<SignupProcessResponse> {
    let res = kg
        .signup
        .init_signup_process(req.username.clone(), req.email.clone(), &req.device_id)
        .await?;

    Ok(response!(SignupProcessResponse {
        expires_at: res.expires_at,
        process_id: res.id,
        next_step: SignupProcessStep::RequirePassword, // TODO: implement other steps
        access_token: None,
    }))
}

#[derive(Deserialize, ToSchema)]
pub struct SignupPasswordRequest {
    device_id: String,
    process_id: String,
    password: String,
}

#[derive(Serialize, ToSchema)]
pub struct SignupPasswordResponse {
    next_step: Option<SignupProcessStep>,
    access_token: Option<String>,
}

#[utoipa::path(
    tag = "Login Process",
    context_path = "/api/v1/process/signup/password",
    request_body = LoginPasswordRequest,
    responses(
        (status = 200, body = SignupPasswordResponse),
        (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
    ),
)]
#[post("/password")]
async fn signup_process_password(req: Json<SignupPasswordRequest>, kg: KG) -> HttpResult {
    let identity = kg
        .signup
        .finish_signup_process(&req.password, &req.process_id, &req.device_id)
        .await?;

    let (refresh_token, access_token) = kg.session.create(&identity.id).await?;
    let cookie = create_refresh_token_cookie(refresh_token, get_refresh_token_cookie_config(kg.config.clone())?)?;
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(response!(SignupPasswordResponse {
            next_step: None,
            access_token: Some(access_token.into()),
        })))
}

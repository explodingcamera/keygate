use actix_web::{
    dev::HttpServiceFactory,
    post,
    web::{self, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    utils::{
        create_refresh_token_cookie, get_refresh_token_cookie_config, response, unauthorized, HttpResult, JsonResult,
    },
    KG,
};

#[derive(Deserialize, ToSchema)]
pub struct LoginProcessRequest {
    username_or_email: String,
    device_id: String,
}

#[derive(Serialize, ToSchema)]
pub enum LoginProcessStep {
    // user needs to enter a password
    RequirePassword,

    // User has 2FA enabled, and we need to verify the code
    Require2fa,

    // Email has to be verified before login
    RequireEmailConfirmation,

    // User has to be approved before login
    RequireEmailVerification,
}

pub fn service(scope: &str) -> impl HttpServiceFactory {
    web::scope(scope)
        .service(create_login_process)
        .service(login_process_password)
}

#[derive(Serialize, ToSchema)]
pub struct LoginProcessResponse {
    expires_at: i64,
    process_id: String,
    next_step: LoginProcessStep,
    access_token: Option<String>,
}

#[utoipa::path(
  tag = "Login Process",
  context_path = "/api/v1/process/login",
  request_body = LoginProcessRequest,
  responses(
      (status = 200, body = LoginProcessResponse),
      (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
  ),
)]
#[post("")]
async fn create_login_process(req: Json<LoginProcessRequest>, kg: KG) -> JsonResult<LoginProcessResponse> {
    let res = kg
        .login
        .init_login_process(&req.username_or_email, &req.device_id)
        .await?;

    Ok(response!(LoginProcessResponse {
        expires_at: res.expires_at,
        process_id: res.id,
        next_step: LoginProcessStep::RequirePassword, // TODO: implement other steps
        access_token: None,
    }))
}

#[derive(Deserialize, ToSchema)]
pub struct LoginPasswordRequest {
    device_id: String,
    process_id: String,
    password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginPasswordResponse {
    next_step: Option<LoginProcessStep>,
    access_token: Option<String>,
}

#[utoipa::path(
    tag = "Login Process",
    context_path = "/api/v1/process/login/password",
    request_body = LoginPasswordRequest,
    responses(
        (status = 200, body = LoginPasswordResponse),
        (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
    ),
)]
#[post("/password")]
async fn login_process_password(req: Json<LoginPasswordRequest>, kg: KG) -> HttpResult {
    let process = kg.login.get_login_process(&req.device_id, &req.process_id).await?;

    let Some(identity) = kg.identity.get_id(&process.process.identity_id).await? else {
        return Err(unauthorized!("identity not found"));
    };

    let Ok(res) = kg.login.validate_password(&req.password, &identity) else {
        return Err(unauthorized!("invalid password"));
    };

    let (refresh_token, access_token) = kg.session.create(&identity.id).await?;
    let cookie = create_refresh_token_cookie(refresh_token, get_refresh_token_cookie_config(kg.config.clone())?)?;
    Ok(HttpResponse::Ok().cookie(cookie).json(response!(LoginPasswordResponse {
        next_step: None,
        access_token: Some(access_token.into()),
    })))
}

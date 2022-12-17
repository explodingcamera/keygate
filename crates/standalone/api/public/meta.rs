use actix_web::{dev::HttpServiceFactory, get, web};
use keygate_core::KeygateError;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    utils::{response, JsonResult},
    KG,
};

pub fn service(scope: &str) -> impl HttpServiceFactory {
    web::scope(scope).service(meta)
}

#[derive(Serialize, ToSchema)]
pub struct MetaResonse {
    signup_enabled: bool,
    login_enabled: bool,
    recovery_enabled: bool,

    login_with_email: bool,
    login_with_username: bool,
    signup_with_email: bool,
    signup_with_username: bool,
    signup_require_email: bool,
    signup_require_email_verification: bool,
    signup_require_username: bool,
    password_min_length: usize,
}

#[utoipa::path(
  tag = "Meta",
  context_path = "/api/v1/meta",
  responses(
      (status = 200, body = MetaResonse),
      (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
  ),
)]
#[get("")]
async fn meta(kg: KG) -> JsonResult<MetaResonse> {
    let cfg = kg
        .config
        .read()
        .map_err(|_| KeygateError::LockPoisoned("config".to_string()))?;

    Ok(response!(MetaResonse {
        signup_enabled: true,
        login_enabled: true,
        recovery_enabled: true,

        login_with_email: cfg.identity.login_with_email,
        login_with_username: cfg.identity.login_with_username,
        signup_with_email: cfg.identity.signup_with_email,
        signup_with_username: cfg.identity.signup_with_username,
        signup_require_email: cfg.identity.signup_require_email,
        signup_require_email_verification: cfg.identity.signup_require_email_verification,
        signup_require_username: cfg.identity.signup_require_username,
        password_min_length: cfg.identity.password_min_length,
    }))
}

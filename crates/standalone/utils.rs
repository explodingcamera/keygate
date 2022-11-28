use actix_web::{
    body::BoxBody,
    cookie::{time::Duration, Cookie, SameSite},
    web::Json,
    HttpResponse,
};
use keygate_core::{config::Environment, utils::tokens::RefreshToken, KeygateConfigInternal};

use crate::errors::KeygateResponseError;

pub type HttpResult<T = BoxBody> = actix_web::Result<HttpResponse<T>, KeygateResponseError>;
pub type JsonResult<T> = actix_web::Result<Json<T>, KeygateResponseError>;

macro_rules! response {
    ($data:expr) => {
        actix_web::web::Json($data)
    };
}

macro_rules! unauthorized {
    ($msg:literal $(,)?) => {
        crate::errors::KeygateResponseError::Unauthorized($msg.to_string())
    };
}

pub(crate) use response;
pub(crate) use unauthorized;

pub struct RefreshTokenCookieOptions {
    pub refresh_token_lifetime: i64,
    pub keygate_domain: String,
    pub refresh_api_path: String,
    pub environment: Environment,
}
pub fn get_refresh_token_cookie_config(
    config: KeygateConfigInternal,
) -> Result<RefreshTokenCookieOptions, KeygateResponseError> {
    let config = config
        .read()
        .map_err(|_| KeygateResponseError::InternalServerError)?
        .clone();

    let keygate_domain = config.server.keygate_domain.clone();
    let public_prefix = config.server.public_prefix.clone().unwrap_or_default();
    let refresh_token_lifetime = config.token.refresh_token_lifetime;

    let refresh_api_path = format!("/{public_prefix}api/v1/session/refresh");

    Ok(RefreshTokenCookieOptions {
        refresh_token_lifetime,
        keygate_domain: config.server.keygate_domain.clone(),
        refresh_api_path,
        environment: config.environment,
    })
}

pub fn create_refresh_token_cookie<'a>(
    refresh_token: RefreshToken,
    opts: RefreshTokenCookieOptions,
) -> Result<Cookie<'a>, KeygateResponseError> {
    if opts.refresh_token_lifetime.is_negative() {
        return Err(KeygateResponseError::InternalServerError);
    }

    let refresh_token_lifetime = Duration::seconds(opts.refresh_token_lifetime);
    let cookie = Cookie::build::<&str, String>("kg_refresh_token", refresh_token.into())
        .domain(opts.keygate_domain)
        .path(opts.refresh_api_path)
        .secure(Environment::Production == opts.environment)
        .same_site(SameSite::None)
        .http_only(true)
        .max_age(refresh_token_lifetime)
        .finish();

    Ok(cookie)
}

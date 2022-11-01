use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    post, HttpRequest, HttpResponse,
};
use keygate_core::config::Environment;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    errors::KeygateResponseError,
    utils::{response, unauthorized, HttpResult},
    KG,
};

#[derive(Serialize, ToSchema)]
pub struct RefreshResponse {
    session_token: String,
}

#[utoipa::path(
    tag = "Session",
    context_path = "/api/v1/session",
    responses(
        (status = 200, body = RefreshResponse),
        (status = 401, body = KeygateErrorResponse, example = json!({"status": 401, "message": "invalid refresh token"}))
    ),
    security(
        ("refresh_token" = [])
    )
)]
#[post("/refresh")]
async fn refresh(req: HttpRequest, kg: KG) -> HttpResult {
    let old_refresh_token = req
        .cookie("kg_refresh_token")
        .ok_or_else(|| unauthorized!("no refresh token"))?;

    old_refresh_token
        .http_only()
        .ok_or_else(|| unauthorized!("invalid refresh token"))?;

    match old_refresh_token.same_site() {
        Some(same_site) => {
            if same_site != actix_web::cookie::SameSite::None {
                return Err(unauthorized!("invalid refresh token"));
            }
        }
        None => return Err(unauthorized!("invalid refresh token")),
    }

    let (keygate_domain, public_prefix, environment, refresh_token_lifetime) = {
        let config = kg
            .config
            .read()
            .map_err(|_| KeygateResponseError::InternalServerError)?;
        (
            config.keygate_domain.clone(),
            config.public_prefix.clone().unwrap_or_default(),
            config.environment.clone(),
            config.refresh_token_lifetime,
        )
    };

    if environment == Environment::Production {
        old_refresh_token
            .secure()
            .ok_or_else(|| unauthorized!("invalid refresh token"))?;
    }

    match old_refresh_token.domain() {
        Some(domain) => {
            if domain != keygate_domain {
                return Err(unauthorized!("invalid refresh token"));
            }
        }
        None => return Err(unauthorized!("invalid refresh token")),
    }

    let refresh_api_path = format!("/{public_prefix}api/v1/session/refresh");
    match old_refresh_token.path() {
        Some(path) => {
            if path != refresh_api_path {
                return Err(unauthorized!("invalid refresh token"));
            }
        }
        None => return Err(unauthorized!("invalid refresh token")),
    }

    let (session_token, refresh_token) = kg.session.refresh(old_refresh_token.value()).await?;
    let session_token: String = session_token.try_into()?;
    if refresh_token_lifetime.is_negative() {
        return Err(KeygateResponseError::InternalServerError);
    }

    let refresh_token_lifetime = Duration::seconds(refresh_token_lifetime);
    let cookie: Cookie = Cookie::build::<&str, String>("kg_refresh_token", refresh_token.into())
        .domain(keygate_domain)
        .path(refresh_api_path)
        .secure(Environment::Production == environment)
        .same_site(SameSite::None)
        .http_only(true)
        .max_age(refresh_token_lifetime)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(response!(RefreshResponse { session_token })))
}

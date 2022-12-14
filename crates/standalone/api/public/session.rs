use actix_web::{dev::HttpServiceFactory, post, web, HttpRequest, HttpResponse};
use keygate_core::config::Environment;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    utils::{create_refresh_token_cookie, get_refresh_token_cookie_config, response, unauthorized, HttpResult},
    KG,
};

pub fn service(scope: &str) -> impl HttpServiceFactory {
    web::scope(scope).service(refresh)
}

#[derive(Serialize, ToSchema)]
pub struct RefreshResponse {
    access_token: String,
}

#[utoipa::path(
    tag = "Session",
    context_path = "/api/v1/session/refresh",
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

    let refresh_token_cookie_config = get_refresh_token_cookie_config(kg.config.clone())?;

    if refresh_token_cookie_config.environment == Environment::Production {
        old_refresh_token
            .secure()
            .ok_or_else(|| unauthorized!("invalid refresh token"))?;
    }

    match old_refresh_token.domain() {
        Some(domain) => {
            if domain != refresh_token_cookie_config.keygate_domain {
                return Err(unauthorized!("invalid refresh token"));
            }
        }
        None => return Err(unauthorized!("invalid refresh token")),
    }

    match old_refresh_token.path() {
        Some(path) => {
            if path != refresh_token_cookie_config.refresh_api_path {
                return Err(unauthorized!("invalid refresh token"));
            }
        }
        None => return Err(unauthorized!("invalid refresh token")),
    }

    let (access_token, refresh_token) = kg.session.refresh(old_refresh_token.value()).await?;
    let access_token: String = access_token.into();
    let cookie = create_refresh_token_cookie(refresh_token, refresh_token_cookie_config)?;

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(response!(RefreshResponse { access_token })))
}

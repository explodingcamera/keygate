use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use keygate_core::{
    api::{
        session::{AccessToken, RefreshToken},
        UserIdentifier,
    },
    Keygate,
};

use crate::errors::AppError;

pub enum AppToken {
    Anon,
    AccessToken(AccessToken),
    RefreshToken(RefreshToken),
}

pub struct ApplicationID(pub String);

const ANON_PREFIX: &str = "Bearer kg0a.";
const ACCESS_PREFIX: &str = "Bearer kg0s.";
const REFRESH_PREFIX: &str = "Bearer kg0r.";

pub async fn validate_token<B>(
    State(keygate): State<Keygate>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let Some(auth_header) = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
    else {
        return Err(AppError::Generic(
            StatusCode::UNAUTHORIZED,
            "Not authenticated",
        ));
    };

    let (token, application_id) = match auth_header {
        h if h.starts_with(ANON_PREFIX) => (
            AppToken::Anon,
            ApplicationID(h.trim_start_matches(ANON_PREFIX).to_owned()),
        ),
        h if h.starts_with("Bearer kg0s") => {
            let token = keygate
                .auth
                .verify_access_token(h.trim_start_matches(ACCESS_PREFIX))
                .await?;
            let application_id = token.audience.clone();
            (AppToken::AccessToken(token), ApplicationID(application_id))
        }
        h if h.starts_with("Bearer kg0r") => {
            let token = keygate
                .auth
                .verify_refresh_token(h.trim_start_matches(REFRESH_PREFIX))
                .await?;
            let application_id = token.audience.clone();
            (AppToken::RefreshToken(token), ApplicationID(application_id))
        }
        _ => {
            return Err(AppError::Generic(
                StatusCode::UNAUTHORIZED,
                "Not authenticated",
            ))
        }
    };

    req.extensions_mut().insert(token);
    req.extensions_mut().insert(application_id);

    Ok(next.run(req).await)
}

pub enum ReqIdentity {
    Anon,
    Identity(keygate_core::database::models::Identity),
    RefreshIdentity(keygate_core::database::models::Identity),
}

pub async fn query_identity<B>(
    State(keygate): State<Keygate>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let Some(app_token) = req.extensions().get::<AppToken>() else {
        return Err(AppError::Generic(
            StatusCode::UNAUTHORIZED,
            "Not authenticated",
        ));
    };

    let identity = match app_token {
        AppToken::Anon => Some(ReqIdentity::Anon),
        AppToken::AccessToken(token) => {
            let identity = keygate
                .identity
                .get(UserIdentifier::Id(token.subject.clone()))
                .await?;
            identity.map(ReqIdentity::Identity)
        }
        AppToken::RefreshToken(token) => {
            let identity = keygate
                .identity
                .get(UserIdentifier::Id(token.subject.clone()))
                .await?;
            identity.map(ReqIdentity::RefreshIdentity)
        }
    };

    if let Some(identity) = identity {
        req.extensions_mut().insert(identity);
    }

    Ok(next.run(req).await)
}

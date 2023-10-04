use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use keygate_core::{api::session::AccessToken, Keygate};

use crate::errors::AppError;

// An extractor that performs authorization.

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(AUTHORIZATION).and_then(|header| header.to_str().ok());

    let _auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(access_token) = Some(AccessToken {
        audience: "".to_owned(),
        subject: "".to_owned(),
        issuer: "".to_owned(),
        session_id: "".to_owned(),
        key_id: "".to_owned(),
    }) {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(access_token);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn identity<B>(
    State(keygate): State<Keygate>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let Some(access_token) = req.extensions().get::<AccessToken>() else {
        return Err(AppError::Generic(StatusCode::UNAUTHORIZED, "Not authenticated"));
    };

    let id = keygate
        .identity
        .get(keygate_core::api::UserIdentifier::Id(access_token.subject.clone()))
        .await?;

    req.extensions_mut().insert(id);
    Ok(next.run(req).await)
}

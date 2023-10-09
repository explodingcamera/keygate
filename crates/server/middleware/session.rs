use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use keygate_core::{api::session::AccessToken, Keygate};

use crate::errors::AppError;

// An extractor that performs authorization.

pub async fn auth<B>(
    State(keygate): State<Keygate>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(AUTHORIZATION).and_then(|header| header.to_str().ok());

    let _auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(access_token) = Some(AccessToken {
        audience: "".to_owned(),
        subject: "anon".to_owned(),
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

pub struct ReqAuth {
    identity: ReqIdentity,
    access_token: AccessToken,
}

pub enum ReqIdentity {
    Anon,
    Identity(keygate_core::database::models::Identity),
}

pub async fn identity<B>(
    State(keygate): State<Keygate>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let Some(access_token) = req.extensions().get::<AccessToken>() else {
        return Err(AppError::Generic(StatusCode::UNAUTHORIZED, "Not authenticated"));
    };

    let auth = match access_token.subject.as_str() {
        "anon" => ReqAuth {
            identity: ReqIdentity::Anon,
            access_token: access_token.clone(),
        },
        id => {
            let Some(identity) = keygate
                .identity
                .get(keygate_core::api::UserIdentifier::Id(id.to_string()))
                .await?
            else {
                return Err(AppError::Generic(StatusCode::UNAUTHORIZED, "Not authenticated"));
            };

            ReqAuth {
                identity: ReqIdentity::Identity(identity),
                access_token: access_token.clone(),
            }
        }
    };

    req.extensions_mut().insert(auth);
    Ok(next.run(req).await)
}

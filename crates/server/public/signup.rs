use axum::extract::{ConnectInfo, State};
use axum::routing::*;
use axum::{Json, Router};

use keygate_core::Keygate;

use crate::errors::AppError;
use std::net::SocketAddr;

pub fn new() -> Router<Keygate> {
    Router::new().route("", post(signup))
}

#[derive(serde::Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(serde::Serialize)]
enum SignupResponse {
    Success {
        access_token: String,
        refresh_token: String,
    },
    RequiresEmailVerification,
}

async fn signup(
    State(keygate): State<Keygate>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    Json(data): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, AppError> {
    let identity = keygate
        .auth
        .signup(&data.username, &data.password, &data.email, ip.ip())
        .await?;

    let (refresh_token, access_token) = keygate.session.create(identity.id).await?;

    Ok(Json(SignupResponse::Success {
        access_token: access_token.0,
        refresh_token: refresh_token.0,
    }))
}

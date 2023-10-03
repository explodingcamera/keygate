use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    routing::post,
    Json, Router,
};
use keygate_core::{api::auth::LoginResponse, Keygate};

use crate::errors::AppError;

pub fn new() -> Router<Keygate> {
    Router::new().route("/login", post(login))
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    username_or_email: String,
}

async fn login(
    State(keygate): State<Keygate>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let res = keygate.auth.login_create(&data.username_or_email, Some(ip.ip())).await?;
    Ok(Json(res))
}

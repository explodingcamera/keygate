use axum::extract::{ConnectInfo, Path, State};
use axum::routing::*;
use axum::{Json, Router};

use keygate_core::api::auth::{LoginResponse, LoginStatusResponse, LoginStep};
use keygate_core::Keygate;

use crate::errors::AppError;
use std::net::SocketAddr;

pub fn new() -> Router<Keygate> {
    Router::new()
        .route("", post(login))
        .route("/step", post(login_step))
        .route("/:process_id", get(login_status))
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
    let res = keygate
        .auth
        .login_create(&data.username_or_email, Some(ip.ip()))
        .await?;
    Ok(Json(res))
}

#[derive(serde::Deserialize)]
struct LoginStepRequest {
    process_id: String,
    step_type: LoginStep,
    data: String,
}

async fn login_step(
    State(keygate): State<Keygate>,
    Json(data): Json<LoginStepRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let res = keygate
        .auth
        .login_step(&data.process_id, data.step_type, &data.data)
        .await?;
    Ok(Json(res))
}

async fn login_status(
    Path(process_id): Path<String>,
    State(keygate): State<Keygate>,
) -> Result<Json<LoginStatusResponse>, AppError> {
    let res = keygate.auth.login_status(&process_id).await?;
    Ok(Json(res))
}

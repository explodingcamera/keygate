use axum::extract::State;
use axum::routing::*;
use axum::{Json, Router};

use keygate_core::Keygate;

use crate::errors::AppError;

pub fn new() -> Router<Keygate> {
    Router::new().route("/validate", post(validate))
}

#[derive(serde::Deserialize)]
pub struct ValidateRequest {
    token: String,
}

#[derive(serde::Serialize)]
pub struct ValidateResponse {
    valid: bool,
}

async fn validate(
    State(keygate): State<Keygate>,
    Json(data): Json<ValidateRequest>,
) -> Result<Json<ValidateResponse>, AppError> {
    Ok(Json(ValidateResponse { valid: true }))
}

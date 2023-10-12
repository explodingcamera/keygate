use axum::extract::State;
use axum::routing::*;
use axum::{Json, Router};

use keygate_core::Keygate;

use crate::errors::AppError;

pub fn new() -> Router<Keygate> {
    Router::new().route("/validate", post(validate))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ValidateRequest {
    token: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct ValidateResponse {
    valid: bool,
}

/// Validate
///
/// Validate a token.
#[utoipa::path(post, path = "/auth/validate", tag = "auth", request_body = ValidateRequest, responses(
    (status = 200, body = ValidateResponse, description = "Token is valid."),
    (status = 400, body = AppError, description = "Invalid request."),
))]
pub(super) async fn validate(
    State(keygate): State<Keygate>,
    Json(data): Json<ValidateRequest>,
) -> Result<Json<ValidateResponse>, AppError> {
    Ok(Json(ValidateResponse { valid: true }))
}

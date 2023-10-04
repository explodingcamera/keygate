use axum::{extract::State, routing::post, Json, Router};
use keygate_core::Keygate;

use crate::errors::AppError;

pub fn new() -> Router<Keygate> {
    Router::new().route("/exists", post(exists))
}

#[derive(serde::Deserialize)]
struct ExistsRequest {
    username_or_email: String,
}

#[derive(serde::Serialize)]
struct ExistsResponse {
    exists: bool,
}

async fn exists(keygate: State<Keygate>, Json(data): Json<ExistsRequest>) -> Result<Json<ExistsResponse>, AppError> {
    let exists = keygate.identity.exists(&data.username_or_email).await?;
    Ok(Json(ExistsResponse { exists }))
}

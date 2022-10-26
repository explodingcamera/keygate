use actix_web::web::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct KeygateResponse<T> {
    pub status: String, // "success" or "error"
    pub data: T,
}

#[derive(ToSchema)]
pub struct RefreshResponse {
    access_token: String,
}

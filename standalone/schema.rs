use actix_web::web::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct RefreshResponse {
    access_token: String,
}

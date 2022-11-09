use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct RefreshResponse {
    access_token: String,
}

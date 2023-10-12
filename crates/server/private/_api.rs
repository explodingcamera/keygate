use super::auth;
use axum::Router;
use keygate_core::Keygate;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(auth::validate),
    components(schemas(auth::ValidateRequest, auth::ValidateResponse, crate::errors::AppError,))
)]
pub struct PrivateAPI;

pub fn new() -> Router<Keygate> {
    Router::new().merge(SwaggerUi::new("/docs/ui").url("/docs/openapi.json", PrivateAPI::openapi()))
}

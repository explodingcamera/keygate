use super::auth;
use axum::Router;
use keygate_core::Keygate;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
  // auth
  auth::validate,
))]
struct PrivateAPI;

pub fn new() -> Router<Keygate> {
    Router::new().merge(SwaggerUi::new("/docs/ui").url("/docs/openapi.json", PrivateAPI::openapi()))
}

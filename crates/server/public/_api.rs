use super::{identity, login, signup};
use axum::Router;
use keygate_core::Keygate;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
  // public::login
  login::login,
  login::login_step,
  login::login_status
))]
struct PublicAPI;

pub fn new() -> Router<Keygate> {
    Router::new().merge(SwaggerUi::new("/docs/ui").url("/docs/openapi.json", PublicAPI::openapi()))
}

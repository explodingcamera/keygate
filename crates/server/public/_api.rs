use super::login;
use axum::Router;
use keygate_core::Keygate;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(login::login, login::login_step, login::login_status),
    components(schemas(
        login::LoginRequest,
        login::LoginStepRequest,
        keygate_core::api::auth::LoginStep,
        keygate_core::api::auth::LoginResponse,
        keygate_core::api::auth::LoginStatusResponse,
        crate::errors::AppError,
    ))
)]
pub struct PublicAPI;

pub fn new() -> Router<Keygate> {
    Router::new().merge(SwaggerUi::new("/docs/ui").url("/docs/openapi.json", PublicAPI::openapi()))
}

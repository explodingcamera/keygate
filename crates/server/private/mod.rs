use axum::Router;
use keygate_core::Keygate;

mod _api;
mod auth;
pub use _api::PrivateAPI;

pub fn new() -> Router<Keygate> {
    Router::new().merge(_api::new()).nest("/auth", auth::new())
}

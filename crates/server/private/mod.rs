use axum::Router;
use keygate_core::Keygate;

mod _api;
mod auth;

pub fn new() -> Router<Keygate> {
    Router::new().merge(_api::new()).nest("/auth", auth::new())
}

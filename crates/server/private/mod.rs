use axum::Router;
use keygate_core::Keygate;

mod auth;

pub fn new() -> Router<Keygate> {
    Router::new().nest("/auth", auth::new())
}

use axum::Router;
use keygate_core::Keygate;

pub fn new() -> Router<Keygate> {
    Router::new()
}

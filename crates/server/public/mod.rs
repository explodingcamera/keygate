use axum::Router;
use keygate_core::Keygate;

mod login;

pub fn new() -> Router<Keygate> {
    Router::new().nest("/auth/login", login::new())
}

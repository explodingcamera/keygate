use axum::Router;
use keygate_core::Keygate;

mod _api;
mod identity;
mod login;
mod signup;

pub fn new() -> Router<Keygate> {
    Router::new()
        .merge(_api::new())
        .nest("/auth/login", login::new())
        .nest("/auth/signup", signup::new())
        .nest("/identity", identity::new())
}

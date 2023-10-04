use axum::Router;
use keygate_core::Keygate;

mod identity;
mod login;
mod signup;

pub fn new() -> Router<Keygate> {
    Router::new()
        .nest("/auth/login", login::new())
        .nest("/auth/signup", signup::new())
        .nest("/identity", identity::new())
}

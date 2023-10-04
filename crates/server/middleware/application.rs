use axum::{
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::errors::AppError;

// An extractor that performs authorization.
pub struct ApplicationID(HeaderValue);
pub async fn application_id<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, AppError> {
    match req.headers().get("X-Application-Id").map(|header| header.clone()) {
        Some(header) => {
            req.extensions_mut().insert(ApplicationID(header));
            Ok(next.run(req).await)
        }
        None => Err(AppError::Generic(
            StatusCode::UNAUTHORIZED,
            "Missing X-Application-Id header",
        )),
    }
}

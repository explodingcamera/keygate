use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use keygate_core::api::APIError;
use serde_json::json;

pub enum AppError {
    APIError(APIError),
    Generic(StatusCode, &'static str),
}

impl From<APIError> for AppError {
    fn from(inner: APIError) -> Self {
        AppError::APIError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Generic(status, message) => (status, message),
            Self::APIError(e) => match e {
                APIError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
                APIError::Cancelled(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cancelled"),
                APIError::Unknown(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown"),
                APIError::InvalidArgument(_) => (StatusCode::BAD_REQUEST, "Invalid argument"),
                APIError::NotFound(_) => (StatusCode::NOT_FOUND, "Not found"),
                APIError::AlreadyExists(_) => (StatusCode::CONFLICT, "Already exists"),
                APIError::PermissionDenied(_) => (StatusCode::FORBIDDEN, "Permission denied"),
                APIError::Aborted(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Aborted"),
                APIError::Unimplemented(_) => (StatusCode::NOT_IMPLEMENTED, "Not implemented"),
                APIError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
                APIError::Unauthenticated(_) => (StatusCode::UNAUTHORIZED, "Unauthenticated"),
            },
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

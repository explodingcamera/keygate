use actix_web::{http::StatusCode, HttpResponse};
use keygate_core::KeygateError;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, ToSchema)]
pub enum KeygateResponseError {
    #[error("internal server error")]
    InternalServerError,

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error(transparent)]
    KeygateError(#[from] KeygateError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl KeygateResponseError {
    fn to_response_data(&self) -> String {
        println!("to_response_data: {:?}", self);

        match *self {
            KeygateResponseError::KeygateError(_) => "Internal Server Error".to_string(), // don't expose internal errors
            KeygateResponseError::IoError(_) => "Internal Server Error".to_string(), // don't expose internal errors
            KeygateResponseError::InternalServerError => "Internal Server Error".to_string(), // don't expose internal errors
            _ => format!("{}", self),
        }
    }
}

impl actix_web::error::ResponseError for KeygateResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            KeygateResponseError::KeygateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        HttpResponse::build(status).json(KeygateErrorResponse {
            status: status.as_u16(),
            message: self.to_response_data(),
        })
    }
}

#[derive(ToSchema, Serialize)]
pub struct KeygateErrorResponse {
    pub status: u16,
    pub message: String,
}

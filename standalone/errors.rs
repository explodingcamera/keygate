use std::fmt::{Display, Formatter};

use actix_web::{http::StatusCode, HttpResponse};
use keygate_core::KeygateError;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::schema::KeygateResponse;

#[derive(Error, Debug, ToSchema)]
pub enum KeygateResponseError {
    #[error("paniced at {0}")]
    Unknown(String),

    #[error(transparent)]
    KeygateError(#[from] KeygateError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl KeygateResponseError {
    fn to_response_data(&self) -> String {
        match *self {
            KeygateResponseError::KeygateError(_) => "Internal Server Error".to_string(), // don't expose internal errors
            KeygateResponseError::IoError(_) => "Internal Server Error".to_string(), // don't expose internal errors
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
        HttpResponse::build(status).json(KeygateResponse::<String> {
            status: "error".to_string(),
            data: self.to_response_data(),
        })
    }
}

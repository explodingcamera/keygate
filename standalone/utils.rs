use actix_web::web::Json;

use crate::{errors::KeygateResponseError, schema::KeygateResponse};

pub type KGResult<T> = actix_web::Result<Json<KeygateResponse<T>>, KeygateResponseError>;

macro_rules! response {
    ($data:expr) => {
        Ok(actix_web::web::Json(KeygateResponse {
            status: "success".to_string(),
            data: $data,
        }))
    };
}

macro_rules! unknown_error {
    ($msg:literal $(,)?) => {
        KGResult::Err(crate::errors::KeygateResponseError::Unknown(
            $msg.to_string(),
        ))
    };
}

pub(crate) use response;
pub(crate) use unknown_error;

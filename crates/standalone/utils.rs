use actix_web::{body::BoxBody, web::Json, HttpResponse};

use crate::errors::KeygateResponseError;

pub type HttpResult<T = BoxBody> = actix_web::Result<HttpResponse<T>, KeygateResponseError>;
pub type JsonResult<T> = actix_web::Result<Json<T>, KeygateResponseError>;

macro_rules! response {
    ($data:expr) => {
        actix_web::web::Json($data)
    };
}

macro_rules! unauthorized {
    ($msg:literal $(,)?) => {
        crate::errors::KeygateResponseError::Unauthorized($msg.to_string())
    };
}

pub(crate) use response;
pub(crate) use unauthorized;

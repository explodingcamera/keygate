use actix_web::{
    dev::HttpServiceFactory,
    put,
    web::{self, Json},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{utils::JsonResult, KG};

pub fn service(scope: &str) -> impl HttpServiceFactory {
    web::scope(scope).service(create_signup_process)
}

#[derive(Deserialize, ToSchema)]
pub struct SignupProcessRequest {}

#[derive(Serialize, ToSchema)]
pub struct SignupProcessResponse {}

#[utoipa::path(
    tag = "Signup Process",
    context_path = "/api/v1/process/signup",
    request_body = SignupProcessRequest,
    responses(
        (status = 200, body = SignupProcessResponse),
        (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
  ),
)]
#[put("/")]
async fn create_signup_process(req: Json<SignupProcessRequest>, kg: KG) -> JsonResult<SignupProcessResponse> {
    todo!()
}

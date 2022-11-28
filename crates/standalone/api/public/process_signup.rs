use actix_web::{put, web::Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{utils::JsonResult, KG};

#[derive(Deserialize, ToSchema)]
pub struct SignupProcessRequest {}

#[derive(Serialize, ToSchema)]
pub struct SignupProcessResponse {}

#[utoipa::path(
  tag = "Signup Process",
  context_path = "/api/v1/signup",
  responses(
      (status = 200, body = SignupProcessResponse),
      (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
  ),
)]
#[put("/")]
async fn create_signup_process(
    req: Json<SignupProcessRequest>,
    kg: KG,
) -> JsonResult<SignupProcessResponse> {
    todo!()
}
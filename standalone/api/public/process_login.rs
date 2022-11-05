use actix_web::{put, web::Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{utils::JsonResult, KG};

#[derive(Deserialize, ToSchema)]
pub struct LoginProcessRequest {
    username_or_email: String,
    device_id: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginProcessResponse {}

#[utoipa::path(
  tag = "Login Process",
  context_path = "/api/v1/login",
  responses(
      (status = 200, body = LoginProcessResponse),
      (status = 401, body = KeygateErrorResponse, example = json!({"status": 400, "message": "invalid json body"}))
  ),
)]
#[put("/")]
async fn create_login_process(
    req: Json<LoginProcessRequest>,
    kg: KG,
) -> JsonResult<LoginProcessResponse> {
    let res = kg
        .login
        .init_login_process(&req.username_or_email, &req.device_id)
        .await?;

    todo!()
}

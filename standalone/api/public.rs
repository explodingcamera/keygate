use actix_web::{get, post, web, Scope};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    errors::KeygateResponseError,
    schema::KeygateResponse,
    utils::{response, unknown_error, KGResult},
    KG,
};

pub fn get() -> Scope {
    let identity = web::scope("/session").service(refresh);
    web::scope("/v1").service(identity)
}

#[derive(Serialize, ToSchema)]
pub struct RefreshResponse {
    session_token: String,
}

#[derive(Serialize, ToSchema)]
struct Pet {
    id: u64,
    name: String,
    age: Option<i32>,
}

#[post("/refresh")]
async fn refresh(kg: KG) -> KGResult<RefreshResponse> {
    let old_refresh_token = "TODO"; // get from cookies
    let (session_token, _refresh_token) = kg.session.refresh(old_refresh_token)?;
    // TODO: set refresh token in cookies

    response!(RefreshResponse {
        session_token: session_token.try_into()?
    })
}

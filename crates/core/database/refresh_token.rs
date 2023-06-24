use std::collections::HashMap;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "refresh_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,

    pub next: Option<String>,
    pub prev: Option<String>,
    pub expires_at: i64,
    pub created_at: i64,
    pub revoked_at: Option<i64>,

    pub session_id: String,
    pub identity_id: String,
    pub access_token_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

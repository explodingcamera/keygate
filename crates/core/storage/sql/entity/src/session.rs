use std::collections::HashMap;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub ip: Option<IpAddr>,
    pub identity_id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub current_refresh_token: String,
    pub revoked_at: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

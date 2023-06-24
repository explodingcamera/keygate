use std::collections::HashMap;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "identities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub username: Option<String>,
    pub primary_email: Option<String>,
    pub password_hash: Option<String>,
    // pub linked_accounts: HashMap<ProviderID, IdentityAccount>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<proto::models::Identity> for Model {
    fn into(self) -> proto::models::Identity {
        proto::models::Identity {
            id: self.id.to_owned(),
            username: self.username.to_owned(),
            primary_email: self.primary_email.to_owned(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            emails: HashMap::default(),
            linked_accounts: HashMap::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::login::Entity")]
    Login,
}

// `Related` trait has to be implemented by hand
impl Related<super::login::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Login.def()
    }
}

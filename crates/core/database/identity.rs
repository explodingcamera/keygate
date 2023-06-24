use std::collections::HashMap;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ToProto;

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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ToProto<proto::models::Identity> for Model {
    fn to_proto_public(&self) -> proto::models::Identity {
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

    fn to_proto_private(&self) -> proto::models::Identity {
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

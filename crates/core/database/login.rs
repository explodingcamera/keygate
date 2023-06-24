use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "login")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub identity_id: String,
    pub ip_address: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
    pub expires_at: i64,
    pub completed: bool,

    pub current_step: String,
    pub magic_link: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::identity::Entity",
        from = "Column::IdentityId",
        to = "super::identity::Column::Id"
    )]
    Identity,
}

// `Related` trait has to be implemented by hand
impl Related<super::identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Identity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use std::sync::Arc;

use crate::{
    database::{models::Identity, DatabasePool},
    KeygateInternal,
};

use keygate_utils::random::secure_random_id;

use super::{APIError, UserIdentifier};

#[derive(Debug, Clone)]
pub struct IdentityAPI {
    keygate: Arc<KeygateInternal>,
}

#[derive(Debug, Clone)]
pub struct CreateIdentity {
    pub username: Option<String>,
    pub primary_email: Option<String>,
    pub password_hash: Option<String>,
}

impl IdentityAPI {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn db(&self) -> &DatabasePool {
        &self.keygate.db
    }

    async fn get(&self, user: UserIdentifier) -> Result<Option<Identity>, APIError> {
        let (field, value) = match user {
            UserIdentifier::Email(email) => ("primary_email", email),
            UserIdentifier::Username(username) => ("username", username),
            UserIdentifier::Id(id) => ("id", id),
        };

        let identity = sqlx::query_as!(Identity, "SELECT * FROM Identity WHERE $1 = $2", field, value)
            .fetch_optional(self.db())
            .await?;

        Ok(identity)
    }

    async fn create(&self, identity: CreateIdentity) -> Result<Identity, APIError> {
        let user_id = secure_random_id();

        Ok(sqlx::query_as!(
            Identity,
            r#"
                INSERT INTO Identity (id, username, primary_email, password_hash)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            user_id,
            identity.username,
            identity.primary_email,
            identity.password_hash
        )
        .fetch_one(self.db())
        .await?)
    }

    async fn update(&self, request: Identity) -> Result<Identity, APIError> {
        // TODO: Implement update_identity function
        unimplemented!()
    }

    async fn delete(&self, id: &str) -> Result<(), APIError> {
        // TODO: Implement delete_identity function
        unimplemented!()
    }

    async fn list(
        &self,
        filters: Vec<proto::api::util::Filter>,
        sort_by: proto::api::util::SortBy,
        sort_order: proto::api::util::SortOrder,
        offset: u32,
        count: u32,
    ) -> Result<Vec<models::Identity>, APIError> {
        if count > 100 {
            return Err(APIError::invalid_argument("Count cannot be greater than 100"));
        }

        let params = filters
            .iter()
            .map(|filter| match filter.field.as_str() {
                "username" => Ok(identity::username::contains(filter.value.clone())),
                "email" => Ok(identity::primary_email::contains(filter.value.clone())),
                _ => Err(APIError::invalid_argument(format!("Invalid filter field: {}", filter.field))),
            })
            .collect::<Result<Vec<prisma::identity::WhereParam>, _>>()?;

        let direction = match sort_order {
            proto::api::util::SortOrder::Asc => SortOrder::Asc,
            proto::api::util::SortOrder::Desc => SortOrder::Desc,
        };

        let order_by = match sort_by {
            proto::api::util::SortBy::Email => identity::primary_email::order,
            proto::api::util::SortBy::Username => identity::username::order,
            proto::api::util::SortBy::CreatedAt => identity::created_at::order,
            proto::api::util::SortBy::LastActive => identity::last_active::order,
        };

        let identities = self
            .client()
            .identity()
            .find_many(params)
            .order_by(order_by(direction))
            .take(count as i64)
            .skip(offset as i64)
            .exec()
            .await?;

        unimplemented!()
    }
}

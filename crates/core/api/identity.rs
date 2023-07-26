use std::sync::Arc;

use crate::{
    database::{models::Identity, DatabasePool},
    KeygateInternal,
};

use keygate_utils::random::secure_random_id;

use super::{APIError, Filter, SortBy, SortOrder, UserIdentifier};

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
        filter: Filter,
        sort_by: SortBy,
        sort_order: SortOrder,
        offset: u32,
        count: u32,
    ) -> Result<Vec<Identity>, APIError> {
        if count > 100 {
            return Err(APIError::invalid_argument("Count cannot be greater than 100"));
        }

        let order_field = match sort_by {
            SortBy::Email => "primary_email",
            SortBy::Username => "username",
            SortBy::CreatedAt => "created_at",
            SortBy::LastActive => "last_active",
        };

        let (filter_field, filter_value) = match filter {
            Filter { ref field, value } if field == "username" => ("username", value),
            Filter { ref field, value } if field == "primary_email" => ("primary_email", value),
            _ => ("id", "".to_string()),
        };

        let identities = match sort_order {
            SortOrder::Asc => {
                sqlx::query_as!(
                    Identity,
                    r#"SELECT * FROM Identity WHERE $1 LIKE $2 ORDER BY $3 ASC LIMIT $4 OFFSET $5"#,
                    filter_field,
                    filter_value,
                    order_field,
                    count,
                    offset
                )
                .fetch_all(self.db())
                .await?
            }
            SortOrder::Desc => {
                sqlx::query_as!(
                    Identity,
                    "SELECT * FROM Identity WHERE $1 LIKE $2 ORDER BY $3 DESC LIMIT $4 OFFSET $5",
                    filter_field,
                    filter_value,
                    order_field,
                    count,
                    offset
                )
                .fetch_all(self.db())
                .await?
            }
        };

        Ok(identities)
    }
}

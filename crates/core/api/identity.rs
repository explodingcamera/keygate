use std::{collections::HashMap, sync::Arc};

use crate::KeygateInternal;
use prisma::{identity, PrismaClient, SortOrder};
use proto::models;

use super::{APIError, UserIdentifier};

#[derive(Debug, Clone)]
pub struct Identity {
    keygate: Arc<KeygateInternal>,
}

impl Identity {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn client(&self) -> &PrismaClient {
        &self.keygate.prisma
    }

    async fn get(&self, user: UserIdentifier) -> Result<models::Identity, APIError> {
        let identity = self
            .client()
            .identity()
            .find_unique(match user {
                UserIdentifier::Email(email) => identity::primary_email::equals(email),
                UserIdentifier::Username(username) => identity::username::equals(username),
                UserIdentifier::Id(id) => identity::id::equals(id),
            })
            .exec()
            .await?
            .ok_or(APIError::not_found("Identity not found"))?;

        Ok(models::Identity {
            id: identity.id,
            username: identity.username,
            primary_email: identity.primary_email,
            created_at: identity.created_at.timestamp(),
            updated_at: identity.updated_at.timestamp(),
            emails: HashMap::default(),
            linked_accounts: HashMap::default(),
        })
    }

    async fn create(&self, request: models::Identity) -> Result<models::Identity, APIError> {
        // TODO: Implement create_identity function
        unimplemented!()
    }

    async fn update(&self, request: models::Identity) -> Result<models::Identity, APIError> {
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

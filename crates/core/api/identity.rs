use std::{collections::HashMap, sync::Arc};

use crate::KeygateInternal;
use prisma::{identity, PrismaClient};
pub use proto::api::identity::identity_service_server::IdentityService;
use proto::{api::identity::*, models};

use super::APIError;

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

    async fn get(&self, request: GetIdentityRequest) -> Result<models::Identity, APIError> {
        let identity = self
            .client()
            .identity()
            .find_unique(match request.user.ok_or(APIError::invalid_argument("User ID is required"))? {
                proto::api::identity::get_identity_request::User::Email(email) => identity::primary_email::equals(email),
                proto::api::identity::get_identity_request::User::Username(username) => identity::username::equals(username),
                proto::api::identity::get_identity_request::User::Id(id) => identity::id::equals(id),
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

    async fn delete(&self, request: DeleteIdentityRequest) -> Result<(), APIError> {
        // TODO: Implement delete_identity function
        unimplemented!()
    }

    async fn list(&self, request: ListIdentitiesRequest) -> Result<ListIdentitiesResponse, APIError> {
        // TODO: Implement list_identities function
        unimplemented!()
    }
}

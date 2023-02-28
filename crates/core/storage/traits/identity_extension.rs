use crate::{models, storage::StorageError};
#[async_trait::async_trait]
pub trait StorageIdentityExtension: Send + Sync {
    async fn identity_create(&self, identity: &models::Identity) -> Result<(), StorageError>;
    async fn identity_update(&self, identity: &models::Identity) -> Result<(), StorageError>;
    async fn identity_by_username(&self, username: &str) -> Result<Option<models::Identity>, StorageError>;
    async fn identity_by_email(&self, email: &str) -> Result<Option<models::Identity>, StorageError>;
    async fn identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError>;
}

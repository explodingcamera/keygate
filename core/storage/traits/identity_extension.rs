use super::StorageSerdeExtension;

use crate::{
    models,
    storage::{BaseStorage, StorageError},
};
#[async_trait::async_trait]
pub trait StorageIdentityExtension: BaseStorage + StorageSerdeExtension + Send + Sync {
    async fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError>;
    async fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError>;
    async fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError>;
    async fn get_identity_by_email(
        &self,
        email: &str,
    ) -> Result<Option<models::Identity>, StorageError>;
    async fn get_identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError>;
}

use super::{GenericKV, StorageSerdeExtension};

use crate::{
    models,
    storage::{constants::*, BaseStorage, LogicStorageError, StorageError},
};

pub trait StorageIdentityExtension: BaseStorage + StorageSerdeExtension + Send + Sync {
    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError>;
    fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError>;
    fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError>;
    fn get_identity_by_email(&self, email: &str) -> Result<Option<models::Identity>, StorageError>;
    fn get_identity_by_id(&self, id: &str) -> Result<Option<models::Identity>, StorageError>;
}

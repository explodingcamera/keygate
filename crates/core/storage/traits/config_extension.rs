use super::StorageSerdeExtension;

use crate::{
    models,
    storage::{BaseStorage, StorageError},
};

#[async_trait::async_trait]
pub trait StorageConfigExtension: BaseStorage + StorageSerdeExtension + Send + Sync {
    async fn get(&self, identity: &models::Identity) -> Result<(), StorageError>;
}

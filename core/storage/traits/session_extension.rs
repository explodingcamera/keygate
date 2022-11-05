use super::StorageSerdeExtension;

use crate::{
    models,
    storage::{BaseStorage, StorageError},
};
#[async_trait::async_trait]
pub trait StorageSessionExtension: BaseStorage + StorageSerdeExtension + Send + Sync {}

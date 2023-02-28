use crate::{models, storage::StorageError};
#[async_trait::async_trait]
pub trait StorageProcessExtension: Send + Sync {
    async fn process_create(&self, process: &models::Process) -> Result<(), StorageError>;
    async fn process_update(&self, updated_process: &models::Process) -> Result<(), StorageError>;
    async fn process_by_id(&self, id: &str) -> Result<Option<models::Process>, StorageError>;
    async fn process_by_token(&self, token_id: &str) -> Result<Option<models::Process>, StorageError>;
}

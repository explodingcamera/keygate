use super::StorageSerdeExtension;

use crate::{
    models,
    storage::{BaseStorage, StorageError},
};
#[async_trait::async_trait]
pub trait StorageProcessExtension: BaseStorage + StorageSerdeExtension + Send + Sync {
    async fn create_process(&self, process: &models::Process) -> Result<(), StorageError> {
        todo!()
    }
    async fn create_process_token(&self, token: &models::ProcessToken) -> Result<(), StorageError> {
        todo!()
    }

    async fn update_process(&self, updated_process: &models::Process) -> Result<(), StorageError> {
        todo!()
    }
    async fn process_by_id(&self, id: &str) -> Result<Option<models::Process>, StorageError> {
        self.process_by_id(id).await
    }
    async fn process_token_by_id(
        &self,
        id: &str,
    ) -> Result<Option<models::ProcessToken>, StorageError> {
        todo!()
    }

    async fn process_by_token(
        &self,
        token_id: &str,
    ) -> Result<Option<models::Process>, StorageError> {
        let token = self.process_token_by_id(token_id).await?;
        match token {
            Some(token) => self.process_by_id(&token.process_id).await,
            None => Ok(None),
        }
    }
}

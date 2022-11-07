use super::StorageSerdeExtension;

use crate::{
    models,
    storage::{BaseStorage, StorageError},
};
#[async_trait::async_trait]
pub trait StorageSessionExtension: BaseStorage + StorageSerdeExtension + Send + Sync {
    async fn get_access_token_by_id(
        &self,
        id: &str,
    ) -> Result<Option<models::AccessToken>, StorageError> {
        todo!()
    }

    async fn get_refresh_token_by_id(
        &self,
        id: &str,
    ) -> Result<Option<models::RefreshToken>, StorageError> {
        todo!()
    }

    async fn get_session_by_id(&self, id: &str) -> Result<Option<models::Session>, StorageError> {
        todo!()
    }

    async fn get_identity_sessions(
        &self,
        identity_id: &str,
    ) -> Result<Option<Vec<models::Session>>, StorageError> {
        todo!()
    }

    async fn create_session(&self, session: &models::Session) -> Result<(), StorageError> {
        todo!()
    }

    async fn create_access_token(
        &self,
        access_token: &models::AccessToken,
    ) -> Result<(), StorageError> {
        todo!()
    }

    async fn create_refresh_token(
        &self,
        refresh_token: &models::RefreshToken,
    ) -> Result<(), StorageError> {
        todo!()
    }

    async fn update_session(&self, session: &models::Session) -> Result<(), StorageError> {
        todo!()
    }

    async fn update_refresh_token(
        &self,
        refresh_token: &models::RefreshToken,
    ) -> Result<(), StorageError> {
        todo!()
    }

    async fn update_access_token(
        &self,
        access_token: &models::AccessToken,
    ) -> Result<(), StorageError> {
        todo!()
    }
}

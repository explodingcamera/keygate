use super::{serde_extension::deserialize, StorageIdentityExtension, StorageSerdeExtension};
use chrono::{DateTime, Utc};

use crate::{
    models::{self, RefreshToken, Session},
    storage::{BaseStorage, StorageError},
    storage_constants::*,
    utils::{self, macros::join_keys},
};
#[async_trait::async_trait]
pub trait StorageSessionExtension:
    BaseStorage + StorageSerdeExtension + StorageIdentityExtension + Send + Sync
{
    async fn refresh_token_by_id(&self, id: &str) -> Result<Option<models::RefreshToken>, StorageError> {
        let bytes = self._get_u8(&join_keys!(REFRESH_TOKEN_BY_ID, id)).await?;
        Ok(deserialize::<RefreshToken>(bytes).await?)
    }

    async fn session_by_id(&self, id: &str) -> Result<Option<models::Session>, StorageError> {
        let bytes = self._get_u8(&join_keys!(SESSION_BY_ID, id)).await?;
        Ok(deserialize::<Session>(bytes).await?)
    }

    async fn get_identity_sessions(&self, identity_id: &str) -> Result<Option<Vec<models::Session>>, StorageError> {
        let bytes = self._get_u8(&join_keys!(IDENTITY_SESSIONS, identity_id)).await?;

        Ok(deserialize::<Vec<Session>>(bytes).await?)
    }

    async fn create_session(
        &self,
        identity_id: &str,
        refresh_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::Session), StorageError>;

    async fn create_refresh_token(&self, refresh_token: &models::RefreshToken) -> Result<(), StorageError> {
        let bytes = utils::serialize::to_bytes(&refresh_token)?;
        self._create_u8(&join_keys!(REFRESH_TOKEN_BY_ID, &refresh_token.id), &bytes)
            .await?;
        Ok(())
    }

    async fn refresh_token(
        &self,
        refresh_token_id: &str,
        refresh_expires_at: DateTime<Utc>,
        access_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::Session), StorageError>;

    async fn revoke_access_token(&self, id: &str) -> Result<(), StorageError>; // requires a transaction

    async fn revoke_refresh_token(&self, id: &str) -> Result<(), StorageError>; // requires a transaction

    async fn reuse_detected(&self, refresh_token: &models::RefreshToken) -> Result<(), StorageError>;
}

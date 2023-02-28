use super::StorageIdentityExtension;
use chrono::{DateTime, Utc};

use crate::{models, storage::StorageError};
#[async_trait::async_trait]
pub trait StorageSessionExtension: StorageIdentityExtension + Send + Sync {
    async fn session_by_id(&self, id: &str) -> Result<Option<models::Session>, StorageError>;
    async fn sessions(&self, identity_id: &str) -> Result<Option<Vec<models::Session>>, StorageError>;
    async fn session_create(
        &self,
        identity_id: &str,
        refresh_expires_at: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::Session), StorageError>;
    async fn session_reuse_detected(&self, refresh_token: &models::RefreshToken) -> Result<(), StorageError>;

    async fn refresh_token_by_id(&self, id: &str) -> Result<Option<models::RefreshToken>, StorageError>;
    async fn refresh_token_create(&self, refresh_token: &models::RefreshToken) -> Result<(), StorageError>;
    async fn refresh_token_revoke(&self, id: &str) -> Result<(), StorageError>; // requires a transaction
    async fn refresh_token_rotate(
        &self,
        id: &str,
        refresh_expires: DateTime<Utc>,
        access_expires: DateTime<Utc>,
    ) -> Result<(models::RefreshToken, models::Session), StorageError>;

    async fn access_token_revoke(&self, id: &str) -> Result<(), StorageError>; // requires a transaction
}

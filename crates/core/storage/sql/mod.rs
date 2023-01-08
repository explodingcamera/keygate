use crate::KeygateConfigInternal;

use super::StorageError;

pub type SQLStorageError = sea_orm::error::DbErr;

pub struct SQLStorage {
    config: KeygateConfigInternal,
}

impl SQLStorage {
    pub async fn new(config: KeygateConfigInternal) -> Result<Self, StorageError> {
        Ok(Self { config })
    }
}

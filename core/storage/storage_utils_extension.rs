use crate::{models, Storage};

use super::{constants::IDENTITY_KEY, LogicStorageError, StorageError, StorageSerdeExtension};

pub trait StorageUtilsExtension: Storage + StorageSerdeExtension + Send + Sync {
    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        self._pset::<models::Identity>(IDENTITY_KEY, identity.id.as_str(), identity)?;
        Ok(())
    }

    fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        if !self.pexists(IDENTITY_KEY, identity.id.as_str())? {
            return Err(LogicStorageError::NotFound(identity.id.clone()).into());
        }

        self._pset::<models::Identity>(IDENTITY_KEY, identity.id.as_str(), identity)?;
        Ok(())
    }
}

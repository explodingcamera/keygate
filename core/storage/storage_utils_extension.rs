use crate::{models, Storage};

use super::{constants::IDENTITY_KEY, StorageError, StorageSerdeExtension};

pub trait StorageUtilsExtension: Storage + StorageSerdeExtension + Send + Sync {
    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        self._pset::<models::Identity>(IDENTITY_KEY, identity.id.as_str(), identity)?;
        Ok(())
    }
}

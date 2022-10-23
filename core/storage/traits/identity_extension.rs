use super::{GenericKV, StorageSerdeExtension};

use crate::{
    models,
    storage::{constants::*, BaseStorage, StorageError},
};

pub trait StorageIdentityExtension: BaseStorage + StorageSerdeExtension + Send + Sync {
    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError>;
    fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError>;
    fn get_identity(&self, id: &str) -> Result<Option<models::Identity>, StorageError>;
    fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError>;
    fn get_identity_by_email(&self, email: &str) -> Result<Option<models::Identity>, StorageError>;
    fn get_identity_by_usern(&self, id: &str) -> Result<Option<models::Identity>, StorageError>;
}

// default implementation for generic storage backends
impl<T: GenericKV> StorageIdentityExtension for T {
    fn get_identity(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        self._pget::<models::Identity>(IDENTITY_BY_ID, id)
    }

    fn get_identity_by_username(
        &self,
        username: &str,
    ) -> Result<Option<models::Identity>, StorageError> {
        let id = self._pget::<String>(IDENTITY_ID_BY_USERNAME, username)?;
        if let Some(id) = id {
            self.get_identity(id.as_str())
        } else {
            Ok(None)
        }
    }

    fn get_identity_by_email(&self, email: &str) -> Result<Option<models::Identity>, StorageError> {
        let id = self._pget::<String>(IDENTITY_ID_BY_EMAIL, email)?;
        if let Some(id) = id {
            self.get_identity(id.as_str())
        } else {
            Ok(None)
        }
    }

    fn get_identity_by_usern(&self, id: &str) -> Result<Option<models::Identity>, StorageError> {
        self._pget::<models::Identity>(IDENTITY_BY_ID, id)
    }

    fn create_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        todo!()
    }

    fn update_identity(&self, identity: &models::Identity) -> Result<(), StorageError> {
        todo!()
    }
}

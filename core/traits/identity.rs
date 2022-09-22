use crate::{models, KeySignal, KeysignalError};

use super::storage_extension::StorageExtension;

static PREFIX: &str = "identity";

pub trait Identity: StorageExtension + Send + Sync {
    fn identity_get(&self, user_id: &str) -> Result<Option<models::Identity>, KeysignalError>;
    fn identity_delete(&self, user_id: &str) -> Result<(), KeysignalError>;
    fn identity_update(
        &self,
        user_id: &str,
        identity: &models::Identity,
    ) -> Result<(), KeysignalError>;
    fn identity_update_in_place<F>(
        &self,
        user_id: &str,
        closure: F,
    ) -> Result<Option<models::Identity>, KeysignalError>
    where
        F: FnOnce(Option<models::Identity>) -> Option<models::Identity>;
    fn identities(&self) -> Result<(), KeysignalError>;
}

impl Identity for KeySignal {
    fn identity_update_in_place<F>(
        &self,
        user_id: &str,
        closure: F,
    ) -> Result<Option<models::Identity>, KeysignalError>
    where
        F: FnOnce(Option<models::Identity>) -> Option<models::Identity>,
    {
        let identity = self.identity_get(user_id)?;
        match closure(identity) {
            Some(new_identity) => {
                self.identity_update(user_id, &new_identity)?;
                Ok(Some(new_identity))
            }
            None => Ok(None),
        }
    }

    fn identity_get(&self, user_id: &str) -> Result<Option<models::Identity>, KeysignalError> {
        Ok(self.pget::<models::Identity>(PREFIX, user_id)?)
    }

    fn identity_delete(&self, _user_id: &str) -> Result<(), KeysignalError> {
        todo!()
    }

    fn identity_update(
        &self,
        user_id: &str,
        identity: &models::Identity,
    ) -> Result<(), KeysignalError> {
        Ok(self.pset(PREFIX, user_id, identity)?)
    }

    fn identities(&self) -> Result<(), KeysignalError> {
        todo!()
    }
}

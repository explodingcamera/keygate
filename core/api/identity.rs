use crate::{models, KeygateConfigInternal, KeygateError, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("unknown error")]
    Unknown,
}

pub struct Identity {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Identity {
    pub fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

pub trait IdentityTrait: Send + Sync {
    fn identity_get(&self, user_id: &str) -> Result<Option<models::Identity>, KeygateError>;
    fn identity_delete(&self, user_id: &str) -> Result<(), KeygateError>;
    fn identity_update(
        &self,
        user_id: &str,
        identity: &models::Identity,
    ) -> Result<(), KeygateError>;
    fn identity_update_in_place<F>(
        &self,
        user_id: &str,
        closure: F,
    ) -> Result<Option<models::Identity>, KeygateError>
    where
        F: FnOnce(Option<models::Identity>) -> Option<models::Identity>;
    fn identities(&self) -> Result<(), KeygateError>;
}

impl IdentityTrait for Identity {
    fn identity_update_in_place<F>(
        &self,
        user_id: &str,
        closure: F,
    ) -> Result<Option<models::Identity>, KeygateError>
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

    fn identity_get(&self, user_id: &str) -> Result<Option<models::Identity>, KeygateError> {
        todo!()
    }

    fn identity_delete(&self, _user_id: &str) -> Result<(), KeygateError> {
        todo!()
    }

    fn identity_update(
        &self,
        user_id: &str,
        identity: &models::Identity,
    ) -> Result<(), KeygateError> {
        Ok(self.storage.update_identity(identity)?)
    }

    fn identities(&self) -> Result<(), KeygateError> {
        todo!()
    }
}

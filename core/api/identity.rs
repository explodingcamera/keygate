use crate::{models, KeygateConfigInternal, KeygateError, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("unknown error {0}")]
    Unknown(String),
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

impl Identity {
    pub fn update_in_place<F>(
        &self,
        user_id: &str,
        closure: F,
    ) -> Result<Option<models::Identity>, KeygateError>
    where
        F: FnOnce(Option<models::Identity>) -> Option<models::Identity>,
    {
        let identity = self.get(user_id)?;
        match closure(identity) {
            Some(new_identity) => {
                if new_identity.id != user_id {
                    return Err(IdentityError::Unknown("identity id mismatch".to_string()).into());
                }

                self.update(&new_identity)?;
                Ok(Some(new_identity))
            }
            None => Ok(None),
        }
    }

    pub fn get(&self, user_id: &str) -> Result<Option<models::Identity>, KeygateError> {
        Ok(self.storage.get_identity_by_id(user_id)?)
    }

    pub fn delete(&self, _user_id: &str) -> Result<(), KeygateError> {
        todo!()
    }

    pub fn update(&self, identity: &models::Identity) -> Result<(), KeygateError> {
        Ok(self.storage.update_identity(identity)?)
    }

    pub fn identities(&self) -> Result<(), KeygateError> {
        todo!()
    }
}

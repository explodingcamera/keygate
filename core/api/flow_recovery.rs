use crate::{KeygateConfigInternal, KeygateError, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecoveryError {
    #[error("unknown error")]
    Unknown,
}

pub struct Recovery {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Recovery {
    pub fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl Recovery {
    pub fn recovery_initiate(&self) -> Result<(), KeygateError> {
        todo!()
    }

    pub fn recovery_complete(&self) -> Result<(), KeygateError> {
        todo!()
    }

    pub fn recovery(&self) -> Result<(), KeygateError> {
        todo!()
    }
}

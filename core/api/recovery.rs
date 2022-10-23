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

pub trait RecoveryTrait: Send + Sync {
    /// Initiate a recovery flow for a user.
    fn recovery_initiate(&self) -> Result<(), KeygateError>;

    /// Complete a recovery flow for a user.
    fn recovery_complete(&self) -> Result<(), KeygateError>;

    /// Get a recovery flow
    fn recovery(&self) -> Result<(), KeygateError>;
}

impl RecoveryTrait for Recovery {
    fn recovery_initiate(&self) -> Result<(), KeygateError> {
        todo!()
    }

    fn recovery_complete(&self) -> Result<(), KeygateError> {
        todo!()
    }

    fn recovery(&self) -> Result<(), KeygateError> {
        todo!()
    }
}

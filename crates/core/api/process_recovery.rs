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
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl Recovery {
    pub async fn init(&self) -> Result<(), KeygateError> {
        todo!()
    }

    pub async fn complete(&self) -> Result<(), KeygateError> {
        todo!()
    }
}

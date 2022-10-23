use crate::{KeygateConfigInternal, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("unknown error")]
    Unknown,
}

pub struct Registration {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Registration {
    pub fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

pub trait RegistrationTrait: Send + Sync {}
impl RegistrationTrait for Registration {}

use crate::{KeygateConfigInternal, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("unknown error")]
    Unknown,
}

pub struct Verification {
    config: KeygateConfigInternal,
    storage: KeygateStorage,
}

impl Verification {
    pub fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

pub trait VerificationTrait: Send + Sync {}
impl VerificationTrait for Verification {}

use crate::Keygate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("unknown error")]
    Unknown,
}

pub trait OAuth: Send + Sync {}

impl OAuth for Keygate {}

use crate::Keygate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("unknown error")]
    Unknown,
}

pub trait Verification: Send + Sync {}
impl Verification for Keygate {}

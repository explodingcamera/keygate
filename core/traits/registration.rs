use crate::Keygate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("unknown error")]
    Unknown,
}

pub trait Registration: Send + Sync {}
impl Registration for Keygate {}

use crate::Keygate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignupError {
    #[error("unknown error")]
    Unknown,
}

pub trait Signup: Send + Sync {}
impl Signup for Keygate {}

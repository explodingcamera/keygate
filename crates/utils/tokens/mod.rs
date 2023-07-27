use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod ed25519;
pub mod formats;
mod keypair;
pub use keypair::*;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error(transparent)]
    Biscuit(#[from] formats::biscuit::BiscuitError),

    #[error(transparent)]
    PasetoError(#[from] formats::paseto::PasetoError),

    #[error(transparent)]
    PasetoClaimError(#[from] formats::paseto::PasetoClaimError),

    #[error("Failed to generate token")]
    FailedToGenerateToken,

    #[error("Invalid token")]
    InvalidToken,
    #[error("Expired token")]
    ExpiredToken,

    #[error("Other error: {0}")]
    Other(String),
}

pub trait SignatureAlgorithm<const PUBLIC_KEY_SIZE: usize, const PRIVATE_KEY_SIZE: usize>: Sized {
    fn try_new(private_key: &[u8]) -> Result<Self, TokenError>;
    fn generate() -> Self;

    fn public_key(&self) -> [u8; PUBLIC_KEY_SIZE];
    fn private_key(&self) -> [u8; PRIVATE_KEY_SIZE];
}

pub trait TokenFormat {
    fn generate_access_token(keypair: KeygateKeypair, token: AccessToken) -> Result<RawAccessToken, TokenError>;
    fn generate_refresh_token(keypair: KeygateKeypair, token: RefreshToken) -> Result<RawRefreshToken, TokenError>;

    fn verify_access_token(public_key: &[u8], token: &str) -> Result<(), TokenError>;
    fn verify_refresh_token(public_key: &[u8], token: &str) -> Result<(), TokenError>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
    Ed25519,
}

pub struct RawAccessToken(pub String);
pub struct RawRefreshToken(pub String);

impl From<String> for RawAccessToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<String> for RawRefreshToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

pub struct AccessToken {}
pub struct RefreshToken {}

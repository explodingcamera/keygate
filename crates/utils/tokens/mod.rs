use serde::{Deserialize, Serialize};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::{encode::ToBase64, random::secure_random_id};

pub mod ed25519;
// pub mod formats;
// pub use biscuit_auth::KeyPair as BiscuitKeypair;

use self::ed25519::Ed25519Keypair;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error(transparent)]
    Biscuit(#[from] biscuit_auth::error::Token),

    #[error("Failed to generate token")]
    FailedToGenerateToken,

    #[error("Invalid token")]
    InvalidToken,
    #[error("Expired token")]
    ExpiredToken,

    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct KeygateKeypair {
    #[zeroize(skip)]
    pub id: String,

    #[zeroize(skip)]
    pub algorithm: Algorithm,

    #[zeroize(skip)]
    inner: InnerKeygateKeypair, // this always has to match the algorithm, otherwise a panic will occur
}

#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
struct KeygateKeypairJson {
    private_key: String,
    #[zeroize(skip)]
    pub id: String,
    #[zeroize(skip)]
    pub public_key: String,
    #[zeroize(skip)]
    pub algorithm: Algorithm,
}

#[derive(Debug, Clone)]
enum InnerKeygateKeypair {
    Ed25519(ed25519::Ed25519Keypair),
}

impl KeygateKeypair {
    pub fn try_from_json(data: &str) -> Result<Self, serde_json::Error> {
        let new = serde_json::from_str::<KeygateKeypairJson>(data)?;

        Ok(Self {
            algorithm: new.algorithm,
            id: new.id.clone(),
            inner: match new.algorithm {
                Algorithm::Ed25519 => InnerKeygateKeypair::Ed25519(
                    ed25519::Ed25519Keypair::try_new(new.private_key.as_bytes()).expect("keypair should be valid"),
                ),
            },
        })
    }

    pub fn to_json(&self) -> Result<String, TokenError> {
        serde_json::to_string(&KeygateKeypairJson {
            algorithm: self.algorithm,
            id: self.id.clone(),
            private_key: self.private_key().to_base64().to_string(),
            public_key: self.public_key().to_base64().to_string(),
        })
        .map_err(|_| TokenError::Other("Failed to serialize keypair".to_string()))
    }

    pub fn generate(algorithm: Algorithm) -> Self {
        let id = secure_random_id();
        Self {
            id,
            algorithm,
            inner: match algorithm {
                Algorithm::Ed25519 => InnerKeygateKeypair::Ed25519(Ed25519Keypair::generate()),
            },
        }
    }

    fn private_key(&self) -> Zeroizing<Vec<u8>> {
        match &self.inner {
            InnerKeygateKeypair::Ed25519(keypair) => keypair.secret_key().to_vec().into(),
        }
    }

    fn public_key(&self) -> Zeroizing<Vec<u8>> {
        match &self.inner {
            InnerKeygateKeypair::Ed25519(keypair) => keypair.public_key().to_vec().into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
    Ed25519,
}

pub struct RawAccessToken(pub String);
pub struct RawRefreshToken(pub String);

pub struct AccessToken {}
pub struct RefreshToken {}

pub trait SignatureAlgorithm<const PUBLIC_KEY_SIZE: usize, const PRIVATE_KEY_SIZE: usize>: Sized {
    fn try_new(private_key: &[u8]) -> Result<Self, TokenError>;
    fn generate() -> Self;

    fn public_key(&self) -> [u8; PUBLIC_KEY_SIZE];
    fn secret_key(&self) -> [u8; PRIVATE_KEY_SIZE];
}

pub trait KeygateToken {
    fn generate_access_token(&self, id: u32, exp: time::OffsetDateTime) -> Result<AccessToken, TokenError>;
    fn generate_refresh_token(&self, id: u32, exp: time::OffsetDateTime) -> Result<RefreshToken, TokenError>;

    fn verify_access_token(token: &str) -> Result<(), TokenError>;
    fn verify_refresh_token(token: &str) -> Result<(), TokenError>;
}

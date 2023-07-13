use biscuit_auth::error::Token;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::encode::{FromBase64, ToBase64};

pub mod biscuit;
pub use biscuit_auth::KeyPair as BiscuitKeypair;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error(transparent)]
    BiscuitToken(#[from] Token),

    #[error("Failed to generate token")]
    FailedToGenerateToken,

    #[error("Invalid token")]
    InvalidToken,
    #[error("Expired token")]
    ExpiredToken,

    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeypairJson {
    id: u32,
    private_key: String,
    public_key: String,
}

pub struct AccessToken(pub String);
pub struct RefreshToken(pub String);

fn to_array<const N: usize>(slice: &[u8]) -> Result<[u8; N], TokenError> {
    slice
        .try_into()
        .map_err(|_| TokenError::Other("Failed to parse private key".to_string()))
}

trait KeygateKeypair<const PUBLIC_KEY_LENGTH: usize, const SECRET_KEY_LENGTH: usize>: Sized {
    fn try_from_json(data: &str) -> Result<(Self, u32), TokenError> {
        let data = serde_json::from_str::<KeypairJson>(data).expect("invalid keypair json");
        let private_key = to_array::<SECRET_KEY_LENGTH>(&data.private_key.decode_base64().expect("invalid private key base64"))
            .expect("invalid private key length");

        Self::try_new(&private_key).map(|keypair| (keypair, data.id))
    }

    fn to_json(&self, id: u32) -> Result<String, TokenError> {
        let private_key = self.secret_key().to_base64();
        let public_key = self.public_key().to_base64();
        let data = KeypairJson {
            id,
            private_key,
            public_key,
        };
        serde_json::to_string(&data).map_err(|_| TokenError::Other("Failed to serialize keypair".to_string()))
    }

    fn try_new(private_key: &[u8; SECRET_KEY_LENGTH]) -> Result<Self, TokenError>;
    fn generate() -> Self;
    fn public_key(&self) -> [u8; PUBLIC_KEY_LENGTH];
    fn secret_key(&self) -> [u8; SECRET_KEY_LENGTH];

    fn get_key_id(token: &str) -> Result<u32, TokenError>;
}

trait KeygateToken<const PUBLIC_KEY_LENGTH: usize, const SECRET_KEY_LENGTH: usize>:
    KeygateKeypair<PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH>
{
    fn generate_access_token(&self, id: u32, exp: u64) -> Result<AccessToken, TokenError>;
    fn generate_refresh_token(&self, id: u32, exp: u64) -> Result<RefreshToken, TokenError>;

    fn verify_access_token(token: &str) -> Result<Self, TokenError>;
    fn verify_refresh_token(token: &str) -> Result<Self, TokenError>;
}

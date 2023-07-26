use super::{
    ed25519::{self, Ed25519Keypair},
    Algorithm, SignatureAlgorithm, TokenError,
};
use crate::{encode::ToBase64, random::secure_random_id};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

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

    pub fn private_key(&self) -> Zeroizing<Vec<u8>> {
        match &self.inner {
            InnerKeygateKeypair::Ed25519(keypair) => keypair.private_key().to_vec().into(),
        }
    }

    pub fn public_key(&self) -> Zeroizing<Vec<u8>> {
        match &self.inner {
            InnerKeygateKeypair::Ed25519(keypair) => keypair.public_key().to_vec().into(),
        }
    }
}

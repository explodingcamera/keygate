use super::{SignatureAlgorithm, TokenError};

#[derive(Debug, Clone)]
pub struct Ed25519Keypair(ed25519_dalek::SigningKey);

impl SignatureAlgorithm<32, 32> for Ed25519Keypair {
    fn generate() -> Self {
        Self(ed25519_dalek::SigningKey::generate(&mut rand_core::OsRng))
    }

    fn try_new(private_key: &[u8]) -> Result<Self, TokenError> {
        Ok(Self(ed25519_dalek::SigningKey::from_bytes(
            private_key
                .try_into()
                .expect("internal private key value should never be invalid"),
        )))
    }

    fn secret_key(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    fn public_key(&self) -> [u8; 32] {
        self.0.verifying_key().to_bytes()
    }
}

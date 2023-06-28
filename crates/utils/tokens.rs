use chrono::{Duration, Utc};
use rusty_paseto::prelude::*;
use thiserror::Error;

use crate::random::secure_random_id;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error(transparent)]
    PasetoError(#[from] PasetoError),

    #[error(transparent)]
    PasetoClaimError(#[from] PasetoClaimError),

    #[error("Failed to generate token")]
    FailedToGenerateToken,

    #[error("Invalid token")]
    InvalidToken,
    #[error("Expired token")]
    ExpiredToken,

    #[error("Other error: {0}")]
    Other(String),
}

pub struct AccessToken(pub String);
pub struct RefreshToken(pub String);

pub struct Keypair {
    pub id: String,
    private_key: Key<64>,
    public_key: Key<32>,
}

pub struct GenericKey(String);
impl<const KEYSIZE: usize> TryFrom<Key<KEYSIZE>> for GenericKey {
    type Error = TokenError;

    fn try_from(value: Key<KEYSIZE>) -> Result<Self, Self::Error> {
        hex::encode(value.as_ref())
            .parse()
            .map_err(|_| TokenError::Other("Failed to parse private key".to_string()))
            .map(Self)
    }
}

pub type PrivateKey = GenericKey;
pub type PublicKey = GenericKey;

impl Keypair {
    pub fn serialize(&self) -> (String, PrivateKey, PublicKey) {
        (
            self.id.clone(),
            self.private_key
                .clone()
                .try_into()
                .expect("internal private key value should never be invalid"),
            self.public_key
                .clone()
                .try_into()
                .expect("internal public key value should never be invalid"),
        )
    }

    pub fn generate() -> Self {
        let private_key = Key::<64>::try_new_random()
            .map_err(|_| TokenError::Other("Failed to generate private key".to_string()))
            .unwrap();

        let public_key = Key::<32>::try_new_random()
            .map_err(|_| TokenError::Other("Failed to generate public key".to_string()))
            .unwrap();

        let key_id = secure_random_id();

        Self {
            id: key_id.clone(),
            private_key,
            public_key,
        }
    }

    pub fn import(id: String, private_key: &str, public_key: &str) -> Result<Self, TokenError> {
        let private_key = Key::<64>::try_from(private_key).map_err(|_| TokenError::Other("Invalid private key".to_string()))?;
        let public_key = Key::<32>::try_from(public_key).map_err(|_| TokenError::Other("Invalid public key".to_string()))?;

        Ok(Self {
            id,
            private_key,
            public_key,
        })
    }

    fn paseto_private_key(&self) -> PasetoAsymmetricPrivateKey<V4, Public> {
        PasetoAsymmetricPrivateKey::<V4, Public>::from(self.private_key.as_slice())
    }

    fn paseto_public_key(&self) -> PasetoAsymmetricPublicKey<V4, Public> {
        PasetoAsymmetricPublicKey::<V4, Public>::from(&self.public_key)
    }

    fn generate_refresh_token(
        &self,
        identity_id: &str,
        session_id: &str,
        audience: &str,
        subject: &str,
        issuer: &str,
    ) -> Result<RefreshToken, TokenError> {
        let in_5_minutes = duration_to_rfc3339(Duration::minutes(5));

        let token_id = secure_random_id();

        let refresh_token = PasetoBuilder::<V4, Public>::default()
            .set_claim(ExpirationClaim::try_from(in_5_minutes)?)
            .set_claim(AudienceClaim::from(audience))
            .set_claim(SubjectClaim::from(subject))
            .set_claim(IssuerClaim::from(issuer))
            .set_claim(TokenIdentifierClaim::from(token_id.as_str()))
            .build(&self.paseto_private_key());

        unimplemented!();
    }

    fn generate_access_token() -> Result<AccessToken, TokenError> {
        unimplemented!();
    }

    fn rotate_refresh_token() -> Result<(RefreshToken, AccessToken), TokenError> {
        unimplemented!();
    }
}

pub fn duration_to_rfc3339(duration: Duration) -> String {
    (Utc::now() + duration).to_rfc3339()
}

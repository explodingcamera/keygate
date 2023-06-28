use crate::random::secure_random_id;
use base64::Engine;
use chrono::{Duration, Utc};
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeypairJson {
    id: String,
    private_key: String,
    public_key: String,
}

impl Keypair {
    pub fn try_from_json(&self, data: &str) -> Result<Self, TokenError> {
        let data = serde_json::from_str::<KeypairJson>(data).expect("should never fail to deserialize keypair");
        Self::try_new(data.id, &data.private_key, &data.public_key)
    }

    pub fn try_new(id: String, private_key: &str, public_key: &str) -> Result<Self, TokenError> {
        let private_key = Key::<64>::try_from(private_key).map_err(|_| TokenError::Other("Invalid private key".to_string()))?;
        let public_key = Key::<32>::try_from(public_key).map_err(|_| TokenError::Other("Invalid public key".to_string()))?;

        Ok(Self {
            id,
            private_key,
            public_key,
        })
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

    pub fn to_json(&self) -> String {
        let private_key: PrivateKey = self
            .private_key
            .clone()
            .try_into()
            .expect("internal private key value should never be invalid");

        let public_key: PublicKey = self
            .public_key
            .clone()
            .try_into()
            .expect("internal public key value should never be invalid");

        serde_json::to_string(&KeypairJson {
            id: self.id.clone(),
            private_key: private_key.0,
            public_key: public_key.0,
        })
        .expect("should never fail to serialize keypair")
    }

    fn paseto_private_key(&self) -> PasetoAsymmetricPrivateKey<V4, Public> {
        PasetoAsymmetricPrivateKey::<V4, Public>::from(self.private_key.as_slice())
    }

    fn paseto_public_key(&self) -> PasetoAsymmetricPublicKey<V4, Public> {
        PasetoAsymmetricPublicKey::<V4, Public>::from(&self.public_key)
    }

    pub fn generate_refresh_token(
        &self,
        // how long is this token valid for?
        duration: Duration,

        // what session is this token for?
        session_id: &str,

        // e.g the web app or the mobile app
        audience: &str,

        // who is this token for? e.g. the identity id
        subject: &str,

        // who issued this token? Node ID
        issuer: &str,
    ) -> Result<RefreshToken, TokenError> {
        let footer = json!({
            "kid": self.id,
        });
        let footer: Footer = footer.as_str().unwrap().into();
        let refresh_token = PasetoBuilder::<V4, Public>::default()
            .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(duration))?)
            .set_claim(AudienceClaim::from(audience))
            .set_claim(SubjectClaim::from(subject))
            .set_claim(IssuerClaim::from(issuer))
            .set_claim(TokenIdentifierClaim::from(session_id))
            .set_claim(CustomClaim::try_from(("kind", "refresh"))?)
            .set_footer(Footer::from(self.id.as_str()))
            .build(&self.paseto_private_key())
            .map_err(|_| TokenError::FailedToGenerateToken)?;
        Ok(RefreshToken(refresh_token))
    }

    pub fn generate_access_token(
        &self,
        // how long is this token valid for?
        duration: Duration,

        // what session is this token for?
        session_id: &str,

        // e.g the web app or the mobile app
        audience: &str,

        // who is this token for? e.g. the identity id
        subject: &str,

        // who issued this token? Node ID
        issuer: &str,
    ) -> Result<AccessToken, TokenError> {
        let access_token = PasetoBuilder::<V4, Public>::default()
            .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(duration))?)
            .set_claim(AudienceClaim::from(audience))
            .set_claim(SubjectClaim::from(subject))
            .set_claim(IssuerClaim::from(issuer))
            .set_claim(TokenIdentifierClaim::from(session_id))
            .set_claim(CustomClaim::try_from(("kind", "access"))?)
            .set_footer(Footer::from(self.id.as_str()))
            .build(&self.paseto_private_key())
            .map_err(|_| TokenError::FailedToGenerateToken)?;

        Ok(AccessToken(access_token))
    }

    pub fn get_key_id(token: &str) -> Result<String, TokenError> {
        // sadly we have to do this because the paseto library doesn't expose the footer
        // without parsing the token first and we need the footer to get the key id to parse the token

        let parts: Vec<&str> = token.split('.').collect();

        if parts.len() != 3 {
            return Err(TokenError::InvalidToken);
        }

        let kid = base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(parts[2].as_bytes())
            .map_err(|_| TokenError::InvalidToken)?;

        let kid = String::from_utf8(kid).map_err(|_| TokenError::InvalidToken)?;

        Ok(kid)
    }
}

pub fn duration_to_rfc3339(duration: Duration) -> String {
    (Utc::now() + duration).to_rfc3339()
}

use crate::{encode::FromBase64, random::secure_random_id};
use chrono::{Duration, Utc};
use rand_core::OsRng;
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};
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
    pub fn try_from_json(data: &str) -> Result<Self, TokenError> {
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
        let key = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let public_key = Key::<32>::from(key.verifying_key().as_bytes());
        let private_key = Key::<64>::from(key.to_keypair_bytes());
        let key_id = secure_random_id();

        Self {
            id: key_id,
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

    pub fn paseto_public_key(&self) -> PasetoAsymmetricPublicKey<V4, Public> {
        PasetoAsymmetricPublicKey::<V4, Public>::from(&self.public_key)
    }

    // After validating the token, the session has to be validated as well and revoked if the refresh token does not match
    pub fn validate_refresh_token(&self, _token: &str) -> Result<(String, String), TokenError> {
        unimplemented!()
    }

    pub fn generate_refresh_token(
        &self,
        // how long is this token valid for?
        duration: Duration,
        // what session is this token for?
        session_id: &str,
    ) -> Result<(RefreshToken, String), TokenError> {
        let token_id = secure_random_id();
        let refresh_token = PasetoBuilder::<V4, Public>::default()
            .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(duration))?)
            .set_claim(AudienceClaim::from("rft"))
            // this is the actual refresh token, the rest is used to prevent replay attacks from old refresh tokens
            .set_claim(TokenIdentifierClaim::from(token_id.as_str()))
            .set_claim(SubjectClaim::from(session_id))
            .set_footer(Footer::from(self.id.as_str()))
            .build(&self.paseto_private_key())
            .map_err(|e| {
                println!("Failed to generate refresh token: {:?}", e);
                TokenError::FailedToGenerateToken
            })?;
        Ok((RefreshToken(refresh_token), token_id))
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
        if audience == "rft" {
            return Err(TokenError::Other("Invalid audience".to_string()));
        }

        let access_token = PasetoBuilder::<V4, Public>::default()
            .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(duration))?)
            .set_claim(AudienceClaim::from(audience))
            .set_claim(SubjectClaim::from(subject))
            .set_claim(IssuerClaim::from(issuer))
            .set_claim(CustomClaim::try_from(("sid", session_id))?)
            .set_claim(CustomClaim::try_from(("kind", "access"))?)
            .set_footer(Footer::from(self.id.as_str()))
            .build(&self.paseto_private_key())
            .map_err(|_| TokenError::FailedToGenerateToken)?;

        Ok(AccessToken(access_token))
    }
}

pub fn get_key_id(token: &str) -> Result<String, TokenError> {
    // sadly we have to do this because the paseto library doesn't expose the footer
    // without parsing the token first and we need the footer to get the key id to parse the token
    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 4 && !parts[0].starts_with("v4") {
        return Err(TokenError::InvalidToken);
    }

    let kid = parts[3].decode_base64_string().map_err(|_| TokenError::InvalidToken)?;
    Ok(kid)
}

pub fn duration_to_rfc3339(duration: Duration) -> String {
    (Utc::now() + duration).to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_token() {
        let keypair = Keypair::generate();
        let (refresh_token, _token_id) = keypair
            .generate_refresh_token(Duration::hours(1), "session_id")
            .expect("failed to generate refresh token");

        let token = refresh_token.0;

        println!("{}", token);

        let keypair = Keypair::try_from_json(&keypair.to_json()).unwrap();
        let kid = get_key_id(&token).unwrap();

        assert_eq!(kid, keypair.id);

        // not implemented yet
        // let (session_id, token_id2) = keypair.validate_refresh_token(&token).unwrap();
        // assert_eq!(session_id, "session_id");
        // assert_eq!(token_id, token_id2);
    }
}

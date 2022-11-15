use keygate_jwt::{
    prelude::{
        Audiences, Claims, Duration, Ed25519KeyPair, EdDSAKeyPairLike, EdDSAPublicKeyLike,
        JWTClaims, NoCustomClaims, NoneAlgorithm, NoneLike, UnixTimeStamp, VerificationOptions,
    },
    JWTError,
};

pub fn generate_session_id() -> String {
    random::secure_random_id()
}

pub fn generate_access_token_id() -> String {
    random::secure_random_id()
}

pub fn generate_refresh_token_id() -> String {
    random::secure_random_id()
}
use super::random;
pub struct RefreshToken(String);

pub enum AccessToken {
    Signed(SignedAccessToken),
    Unsigned(UnsignedAccessToken),
}

impl ToString for AccessToken {
    fn to_string(&self) -> String {
        match self {
            AccessToken::Signed(token) => token.0.clone(),
            AccessToken::Unsigned(token) => token.0.clone(),
        }
    }
}

pub struct KeygateClaims {
    pub issued_at: UnixTimeStamp,
    pub expires_at: UnixTimeStamp,
    pub issuer: String,
    pub subject: String,
    pub audiences: Audiences,
    pub jwt_id: String,
}

impl TryInto<KeygateClaims> for JWTClaims<NoCustomClaims> {
    type Error = JWTError;

    fn try_into(self) -> Result<KeygateClaims, Self::Error> {
        if let (
            Some(issued_at),
            Some(expires_at),
            Some(issuer),
            Some(subject),
            Some(audiences),
            Some(jwt_id),
        ) = (
            self.issued_at,
            self.expires_at,
            self.issuer,
            self.subject,
            self.audiences,
            self.jwt_id,
        ) {
            Ok(KeygateClaims {
                issued_at,
                expires_at,
                issuer,
                subject,
                audiences,
                jwt_id,
            })
        } else {
            Err(JWTError::NotJWT)
        }
    }
}

impl From<UnsignedAccessToken> for AccessToken {
    fn from(token: UnsignedAccessToken) -> Self {
        AccessToken::Unsigned(token)
    }
}

impl From<SignedAccessToken> for AccessToken {
    fn from(token: SignedAccessToken) -> Self {
        AccessToken::Signed(token)
    }
}

pub struct SignedAccessToken(String);

impl SignedAccessToken {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn verify(&self, key_pair: &Ed25519KeyPair) -> Result<KeygateClaims, JWTError> {
        let options = VerificationOptions {
            ..Default::default()
        };
        let claims: JWTClaims<NoCustomClaims> =
            key_pair.public_key().verify_token(&self.0, Some(options))?;
        claims.try_into()
    }

    pub fn generate(
        user_id: &str,
        audience: &str,
        expires_in_seconds: u64,
        key_pair: Ed25519KeyPair,
    ) -> Result<Self, JWTError> {
        let claims = Claims::create(Duration::from_secs(expires_in_seconds))
            .with_issuer("keygate")
            .with_audience(audience)
            .with_subject(user_id)
            .with_jwt_id(generate_access_token_id());

        let token = key_pair.sign(claims)?;
        Ok(SignedAccessToken(token))
    }
}

pub struct UnsignedAccessToken(String);

impl UnsignedAccessToken {
    pub fn new(access_token_id: &str) -> Self {
        Self(access_token_id.to_string())
    }

    pub fn parse(&self) -> Result<KeygateClaims, JWTError> {
        let options = VerificationOptions {
            ..Default::default()
        };

        let none = NoneAlgorithm::new();
        #[allow(unsafe_code)]
        let claims = unsafe { none.parse_token(&self.0, Some(options)) }?;
        claims.try_into()
    }

    pub fn generate(
        user_id: &str,
        audience: &str,
        expires_in_seconds: u64,
    ) -> Result<Self, JWTError> {
        let none = NoneAlgorithm::new();
        let claims = Claims::create(Duration::from_secs(expires_in_seconds))
            .with_issuer("keygate")
            .with_audience(audience)
            .with_subject(user_id)
            .with_jwt_id(generate_access_token_id());

        #[allow(unsafe_code)]
        let token = unsafe { none.create(claims) }?;

        Ok(UnsignedAccessToken(token))
    }
}

impl RefreshToken {
    pub fn new(refresh_token_id: &str) -> Self {
        Self(refresh_token_id.to_string())
    }
}

impl From<RefreshToken> for String {
    fn from(token: RefreshToken) -> Self {
        token.0
    }
}

impl From<String> for RefreshToken {
    fn from(token: String) -> Self {
        RefreshToken(token)
    }
}

impl From<String> for SignedAccessToken {
    fn from(token: String) -> Self {
        SignedAccessToken(token)
    }
}
impl From<String> for UnsignedAccessToken {
    fn from(token: String) -> Self {
        UnsignedAccessToken(token)
    }
}

// test this
#[cfg(test)]
mod tests {
    use super::*;
    use keygate_jwt::prelude::Ed25519KeyPair;

    #[test]
    fn test_refresh_token() {
        let token = RefreshToken::new(&generate_refresh_token_id());
        assert_eq!(token.0.len(), 21);
    }

    #[test]
    fn test_unsigned_access_token() {
        let token = UnsignedAccessToken::generate("user_id", "audience", 3600).unwrap();
        assert_eq!(token.0.len(), 212);
        let claims = token.parse().unwrap();
        assert_eq!(claims.issuer, "keygate");
        assert_eq!(claims.subject, "user_id");
        assert_eq!(
            claims.audiences,
            Audiences::AsString("audience".to_string())
        );
    }

    #[test]
    fn test_signed_access_token() {
        let key_pair = Ed25519KeyPair::generate();
        let token =
            SignedAccessToken::generate("user_id", "audience", 3600, key_pair.clone()).unwrap();
        assert_eq!(token.0.len(), 299);
        let claims = token.verify(&key_pair).unwrap();
        assert_eq!(claims.issuer, "keygate");
        assert_eq!(claims.subject, "user_id");
        assert_eq!(
            claims.audiences,
            Audiences::AsString("audience".to_string())
        );
    }
}

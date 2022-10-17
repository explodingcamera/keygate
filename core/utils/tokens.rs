use serde::{Deserialize, Serialize};
use std::io;

use super::random;
pub struct RefreshToken(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenHeader {
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenPayload {
    // The issuer of the token (we)
    pub iss: String,

    // The user id
    pub sub: String,

    // The audience of the token (the client)
    pub aud: String,

    // expiration time
    pub exp: u64,

    // issued at
    pub iat: u64,

    // session token id
    pub jti: String,
}

// SessionToken is a JWT token that is used to authenticate a user
// and authorize them to access a resource.
// The token is currently not signed or encrypted, since we don't
// need to verify the authenticity of the token because we only use
// the jti (session token id) to look up the session in the database.
// All the other fields are only for use by the client.
// That's also why Refresh Token is not a JWT token, since it is
// not used by the client.
pub struct SessionToken {
    pub header: SessionTokenHeader,
    pub payload: SessionTokenPayload,
}

impl TryFrom<SessionToken> for String {
    type Error = io::Error;
    fn try_from(token: SessionToken) -> io::Result<Self> {
        let header = base64::encode(serde_json::to_string(&token.header)?);
        let payload = base64::encode(serde_json::to_string(&token.payload)?);
        Ok(format!("{header}.{payload}."))
    }
}

impl TryFrom<String> for SessionToken {
    type Error = io::Error;
    fn try_from(token: String) -> io::Result<Self> {
        let parts: Vec<&str> = token.split('.').collect();

        let header = base64::decode(parts[0])
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid token"))?;
        let header = serde_json::from_slice(&header)?;

        let payload = base64::decode(parts[1])
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid token"))?;
        let payload = serde_json::from_slice(&payload)?;

        Ok(SessionToken { header, payload })
    }
}

impl SessionToken {
    pub fn new(user_id: &str, audience: &str, expires_in_seconds: u64) -> Self {
        let jti = random::secure_random_id();
        let now = chrono::Utc::now().timestamp() as u64;
        let exp = now + expires_in_seconds; // 7 days

        let header = SessionTokenHeader {
            typ: "JWT".to_string(),
        };

        let payload = SessionTokenPayload {
            iss: "keygate".to_string(),
            sub: user_id.to_string(),
            aud: audience.to_string(),
            exp,
            iat: now,
            jti,
        };

        SessionToken { header, payload }
    }
}

impl RefreshToken {
    pub fn new() -> Self {
        let token = random::secure_random_id();
        RefreshToken(token)
    }
}

impl Default for RefreshToken {
    fn default() -> Self {
        RefreshToken::new()
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

use biscuit_auth::{KeyPair, PrivateKey, UnverifiedBiscuit};

use super::{KeygateKeypair, KeygateToken};

impl KeygateToken<32, 32> for KeyPair {
    fn generate_access_token(&self, _id: u32, _exp: time::OffsetDateTime) -> Result<super::AccessToken, super::TokenError> {
        todo!()
    }

    fn generate_refresh_token(&self, _id: u32, _exp: time::OffsetDateTime) -> Result<super::RefreshToken, super::TokenError> {
        todo!()
    }

    fn verify_access_token(_token: &str) -> Result<Self, super::TokenError> {
        todo!()
    }

    fn verify_refresh_token(_token: &str) -> Result<Self, super::TokenError> {
        todo!()
    }
}

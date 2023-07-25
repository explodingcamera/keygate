use biscuit_auth::{KeyPair, PrivateKey, UnverifiedBiscuit};

use super::{KeygateKeypair, KeygateToken};

impl KeygateKeypair<32, 32> for KeyPair {
    fn try_new(private_key: &[u8; 32]) -> Result<Self, super::TokenError> {
        let private_key = PrivateKey::from_bytes(private_key)
            .map_err(|_| super::TokenError::Other("Failed to parse private key".to_string()))?;
        Ok(Self::from(&private_key))
    }

    fn generate() -> Self {
        KeyPair::new()
    }

    fn public_key(&self) -> [u8; 32] {
        self.kp.public.to_bytes()
    }

    fn secret_key(&self) -> [u8; 32] {
        self.kp.secret.to_bytes()
    }

    fn get_key_id(token: &str) -> Result<u32, super::TokenError> {
        let token = UnverifiedBiscuit::from_base64(token).map_err(|_| super::TokenError::InvalidToken)?;
        token.root_key_id().ok_or(super::TokenError::InvalidToken)
    }
}

impl KeygateToken<32, 32> for KeyPair {
    fn generate_access_token(&self, _id: u32, _exp: u64) -> Result<super::AccessToken, super::TokenError> {
        todo!()
    }

    fn generate_refresh_token(&self, _id: u32, _exp: u64) -> Result<super::RefreshToken, super::TokenError> {
        todo!()
    }

    fn verify_access_token(_token: &str) -> Result<Self, super::TokenError> {
        todo!()
    }

    fn verify_refresh_token(_token: &str) -> Result<Self, super::TokenError> {
        todo!()
    }
}

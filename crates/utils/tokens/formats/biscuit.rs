use crate::tokens::*;
pub use biscuit_auth::error::Token as BiscuitError;
use biscuit_auth::macros::biscuit;

pub struct Biscuit();

impl Biscuit {
    fn to_biscuit_keypair(keypair: KeygateKeypair) -> biscuit_auth::KeyPair {
        let private_key = biscuit_auth::PrivateKey::from_bytes(keypair.private_key().as_slice())
            .expect("Failed to parse private key, this is a bug");
        biscuit_auth::KeyPair::from(&private_key)
    }
}

impl TokenFormat for Biscuit {
    fn generate_access_token(
        keypair: KeygateKeypair,
        _token: GenerateAccessToken,
    ) -> Result<RawAccessToken, TokenError> {
        let keypair = Self::to_biscuit_keypair(keypair);
        let biscuit = biscuit!(r#""#).build(&keypair)?;
        Ok(biscuit.to_string().into())
    }

    fn generate_refresh_token(
        keypair: KeygateKeypair,
        _token: GenerateRefreshToken,
    ) -> Result<RawRefreshToken, TokenError> {
        let keypair = Self::to_biscuit_keypair(keypair);
        let biscuit = biscuit!(r#""#).build(&keypair)?;
        Ok(biscuit.to_string().into())
    }

    fn verify_access_token(_public_key: &[u8], _token: &str) -> Result<AccessToken, TokenError> {
        todo!()
    }

    fn verify_refresh_token(_public_key: &[u8], _token: &str) -> Result<RefreshToken, TokenError> {
        todo!()
    }
}

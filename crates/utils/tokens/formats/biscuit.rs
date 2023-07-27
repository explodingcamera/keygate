use crate::tokens::*;
pub use biscuit_auth::error::Token as BiscuitError;
use biscuit_auth::macros::biscuit;

struct Biscuit();

impl Biscuit {
    fn to_biscuit_keypair(keypair: KeygateKeypair) -> biscuit_auth::KeyPair {
        let private_key = biscuit_auth::PrivateKey::from_bytes(keypair.private_key().as_slice())
            .expect("Failed to parse private key, this is a bug");
        biscuit_auth::KeyPair::from(&private_key)
    }
}

impl TokenFormat for Biscuit {
    fn generate_access_token(keypair: KeygateKeypair, _token: AccessToken) -> Result<RawAccessToken, TokenError> {
        let keypair = Self::to_biscuit_keypair(keypair);
        let biscuit = biscuit!(r#""#).build(&keypair)?;
        Ok(biscuit.to_string().into())
    }

    fn generate_refresh_token(keypair: KeygateKeypair, _token: RefreshToken) -> Result<RawRefreshToken, TokenError> {
        let keypair = Self::to_biscuit_keypair(keypair);
        let biscuit = biscuit!(r#""#).build(&keypair)?;
        Ok(biscuit.to_string().into())
    }

    fn verify_access_token(public_key: &[u8], token: &str) -> Result<(), TokenError> {
        todo!()
    }

    fn verify_refresh_token(public_key: &[u8], token: &str) -> Result<(), TokenError> {
        todo!()
    }
}

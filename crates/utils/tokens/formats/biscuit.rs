use crate::tokens::*;
pub use biscuit_auth::error::Token as BiscuitError;

struct Biscuit();

impl Biscuit {}

impl TokenFormat for Biscuit {
    fn generate_access_token(keypair: KeygateKeypair, token: AccessToken) -> Result<RawAccessToken, TokenError> {
        todo!()
    }

    fn generate_refresh_token(keypair: KeygateKeypair, token: RefreshToken) -> Result<RawRefreshToken, TokenError> {
        todo!()
    }

    fn verify_access_token(public_key: &[u8], token: &str) -> Result<(), TokenError> {
        todo!()
    }

    fn verify_refresh_token(public_key: &[u8], token: &str) -> Result<(), TokenError> {
        todo!()
    }
}

use crate::{encode::FromBase64, random::secure_random_id, tokens::*};
use rusty_paseto::prelude::*;

pub use rusty_paseto::prelude::{PasetoClaimError, PasetoError};
use time::{format_description::well_known::Rfc3339, Duration};

struct Paseto();

impl TokenFormat for Paseto {
    fn generate_access_token(keypair: KeygateKeypair, token: AccessToken) -> Result<RawAccessToken, TokenError> {
        let key = PasetoAsymmetricPrivateKey::<V4, Public>::from(keypair.private_key().as_slice());

        // let access_token = PasetoBuilder::<V4, Public>::default()
        //     .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(duration))?)
        //     .set_claim(AudienceClaim::from(audience))
        //     .set_claim(SubjectClaim::from(subject))
        //     .set_claim(IssuerClaim::from(issuer))
        //     .set_claim(CustomClaim::try_from(("sid", session_id))?)
        //     .set_claim(CustomClaim::try_from(("kind", "access"))?)
        //     .set_footer(Footer::from(self.id.as_str()))
        //     .build(&self.paseto_private_key())
        //     .map_err(|_| TokenError::FailedToGenerateToken)?;
        unimplemented!()
    }

    fn generate_refresh_token(keypair: KeygateKeypair, token: RefreshToken) -> Result<RawRefreshToken, TokenError> {
        let key = PasetoAsymmetricPrivateKey::<V4, Public>::from(keypair.private_key().as_slice());

        // let token_id = secure_random_id();
        // let refresh_token = PasetoBuilder::<V4, Public>::default()
        //     .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(duration))?)
        //     .set_claim(AudienceClaim::from("rft"))
        //     // this is the actual refresh token, the rest is used to prevent replay attacks from old refresh tokens
        //     .set_claim(TokenIdentifierClaim::from(token_id.as_str()))
        //     .set_claim(SubjectClaim::from(session_id))
        //     .set_footer(Footer::from(self.id.as_str()))
        //     .build(&self.paseto_private_key())
        //     .map_err(|e| {
        //         println!("Failed to generate refresh token: {:?}", e);
        //         TokenError::FailedToGenerateToken
        //     })?;
        unimplemented!()
    }

    fn verify_access_token(public_key: &[u8], token: &str) -> Result<(), TokenError> {
        todo!()
    }

    fn verify_refresh_token(public_key: &[u8], token: &str) -> Result<(), TokenError> {
        todo!()
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
    let now = time::OffsetDateTime::now_utc();
    let expiration = now + duration;
    expiration.format(&Rfc3339).unwrap()
}

#[cfg(test)]
mod tests {}

use crate::{encode::FromBase64, tokens::*};
use rusty_paseto::prelude::*;

pub use rusty_paseto::prelude::{PasetoClaimError, PasetoError};
use time::{format_description::well_known::Rfc3339, Duration};

pub struct Paseto();

impl TokenFormat for Paseto {
    fn generate_access_token(
        keypair: KeygateKeypair,
        token: GenerateAccessToken,
    ) -> Result<RawAccessToken, TokenError> {
        let key = keypair.private_key();
        let key = PasetoAsymmetricPrivateKey::<V4, Public>::from(key.as_slice());

        let access_token = PasetoBuilder::<V4, Public>::default()
            .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(
                token.duration,
            ))?)
            .set_claim(AudienceClaim::from(token.audience.as_str()))
            .set_claim(SubjectClaim::from(token.subject.as_str()))
            .set_claim(IssuerClaim::from(token.issuer.as_str()))
            .set_claim(CustomClaim::try_from(("sid", token.session_id))?)
            .set_claim(CustomClaim::try_from(("kind", "access"))?)
            .set_footer(Footer::from(keypair.id.as_str()))
            .build(&key)
            .map_err(|_| TokenError::FailedToGenerateToken)?;

        Ok(RawAccessToken(access_token))
    }

    fn generate_refresh_token(
        keypair: KeygateKeypair,
        token: GenerateRefreshToken,
    ) -> Result<RawRefreshToken, TokenError> {
        let key = keypair.private_key();
        let key = PasetoAsymmetricPrivateKey::<V4, Public>::from(key.as_slice());

        let refresh_token = PasetoBuilder::<V4, Public>::default()
            .set_claim(ExpirationClaim::try_from(duration_to_rfc3339(
                token.duration,
            ))?)
            .set_claim(AudienceClaim::from(token.audience.as_str()))
            .set_claim(SubjectClaim::from(token.subject.as_str()))
            .set_claim(IssuerClaim::from(token.issuer.as_str()))
            .set_claim(CustomClaim::try_from(("sid", token.session_id))?)
            .set_claim(CustomClaim::try_from(("kind", "refresh"))?)
            .set_footer(Footer::from(keypair.id.as_str()))
            .build(&key)
            .map_err(|_| TokenError::FailedToGenerateToken)?;

        Ok(RawRefreshToken(refresh_token))
    }

    fn verify_access_token(public_key: &[u8], token: &str) -> Result<AccessToken, TokenError> {
        let key: Key<32> = public_key
            .try_into()
            .map_err(|_| TokenError::InvalidToken)?;
        let key = PasetoAsymmetricPublicKey::<V4, Public>::from(&key);

        let claims = PasetoParser::<V4, Public>::default()
            .check_claim(CustomClaim::try_from(("kind", "access"))?)
            .parse(token, &key)
            .map_err(|_| TokenError::InvalidToken)?;

        Ok(AccessToken {
            audience: claims["aud"].to_string(),
            subject: claims["sub"].to_string(),
            issuer: claims["iss"].to_string(),
            session_id: claims["sid"].to_string(),
            key_id: claims["kid"].to_string(),
        })
    }

    fn verify_refresh_token(public_key: &[u8], token: &str) -> Result<RefreshToken, TokenError> {
        let key: Key<32> = public_key
            .try_into()
            .map_err(|_| TokenError::InvalidToken)?;
        let key = PasetoAsymmetricPublicKey::<V4, Public>::from(&key);

        let claims = PasetoParser::<V4, Public>::default()
            .check_claim(CustomClaim::try_from(("kind", "refresh"))?)
            .parse(token, &key)
            .map_err(|_| TokenError::InvalidToken)?;

        Ok(RefreshToken {
            audience: claims["aud"].to_string(),
            subject: claims["sub"].to_string(),
            issuer: claims["iss"].to_string(),
            session_id: claims["sid"].to_string(),
            key_id: claims["kid"].to_string(),
        })
    }
}

pub fn get_key_id(token: &str) -> Result<String, TokenError> {
    // sadly we have to do this because the paseto library doesn't expose the footer
    // without parsing the token first and we need the footer to get the key id to parse the token
    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 4 && !parts[0].starts_with("v4") {
        return Err(TokenError::InvalidToken);
    }

    let kid = parts[3]
        .decode_base64_string()
        .map_err(|_| TokenError::InvalidToken)?;
    Ok(kid)
}

pub fn duration_to_rfc3339(duration: Duration) -> String {
    let now = time::OffsetDateTime::now_utc();
    let expiration = now + duration;
    expiration.format(&Rfc3339).unwrap()
}

#[cfg(test)]
mod tests {}

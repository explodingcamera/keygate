use chrono::Utc;
use proto::models::RefreshToken;

use super::random::RANDOMID_ALPHABET;

pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 8
}

pub fn is_valid_id(random_string: &str) -> bool {
    random_string.len() == 21 && random_string.chars().all(|c| RANDOMID_ALPHABET.contains(&c))
}

pub fn is_valid_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    re.is_match(email)
}

pub fn is_valid_username(username: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+$").unwrap();
    re.is_match(username) && username.len() >= 3 && username.len() <= 32
}

#[derive(thiserror::Error, Debug)]
pub enum RefreshTokenError {
    #[error(transparent)]
    ReuseError(#[from] RefreshTokenReuseError),

    #[error("refresh token expired")]
    Expired,

    #[error("invalid refresh token")]
    Invalid,
}

#[derive(thiserror::Error, Debug)]
pub enum RefreshTokenReuseError {
    #[error("Refresh token revoked")]
    Revoked,
    #[error("Refresh token superceeded")]
    Superceeded,
}

pub fn can_refresh(refresh_token: &RefreshToken) -> Result<(), RefreshTokenError> {
    if refresh_token.next.is_some() {
        return Err(RefreshTokenReuseError::Superceeded.into());
    }

    if refresh_token.revoked_at.is_some() {
        return Err(RefreshTokenReuseError::Revoked.into());
    }

    let now = Utc::now();
    if refresh_token.expires_at < now.timestamp() {
        return Err(RefreshTokenError::Expired);
    }

    if refresh_token.created_at > now.timestamp() {
        return Err(RefreshTokenError::Invalid);
    }

    if !is_valid_id(&refresh_token.id)
        || !is_valid_id(&refresh_token.access_token_id)
        || !is_valid_id(&refresh_token.session_id)
    {
        return Err(RefreshTokenError::Invalid);
    }

    Ok(())
}

pub fn can_refresh_session(session: &crate::models::Session) -> bool {
    if session.revoked_at.is_some() {
        return false;
    }

    if !is_valid_id(&session.id) || !is_valid_id(&session.identity_id) {
        return false;
    }

    true
}

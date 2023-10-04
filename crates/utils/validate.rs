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

pub fn validate_field<E>(
    field: &Option<String>,
    required: bool,
    validator: impl Fn(&str) -> bool,
    error: E,
) -> Result<(), E> {
    if required && field.is_none() {
        return Err(error);
    }

    if let Some(value) = field {
        if !validator(value) {
            return Err(error);
        }
    }

    Ok(())
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

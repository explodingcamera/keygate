use super::sha1::hash;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PwnedError {
    #[error("{0}")]
    Internal(String),
    #[error("Passwords Been Pwned")]
    Pwned,
}

static PWNED_URL: &str = "https://api.pwnedpasswords.com/range/";

pub async fn pwned_check(password: &str) -> Result<(), PwnedError> {
    let pw_hash = hash(password);
    let pw_str = hex::encode_upper(pw_hash);

    let (prefix, suffex) = pw_str.split_at(5);
    let response = reqwest::get(PWNED_URL.to_owned() + prefix)
        .await
        .map_err(|e| PwnedError::Internal("reqwest failed: ".to_string() + &e.to_string()))?;
    let response_body = response
        .text()
        .await
        .map_err(|e| PwnedError::Internal("text error: ".to_string() + &e.to_string()))?;

    if response_body.split('\n').any(|row| row.starts_with(suffex)) {
        return Err(PwnedError::Pwned);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::pwned_check;
    use super::PwnedError;

    #[tokio::test]
    async fn test_pwned_check() -> Result<(), PwnedError> {
        assert_eq!(
            Err(PwnedError::Pwned),
            pwned_check("password").await,
            "password is pwned"
        );
        assert_eq!(
            Err(PwnedError::Pwned),
            pwned_check("adminadmin").await,
            "adminadmin is pwned"
        );
        assert_eq!(
            Ok(()),
            pwned_check("flkatoihkvdjnasdjölewm").await,
            "flkatoihkvdjnasdjölewm is not pwned"
        );
        Ok(())
    }
}

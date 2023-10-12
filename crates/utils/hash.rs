use argon2::{password_hash::SaltString, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

pub fn password(password: &str) -> std::io::Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = argon2::Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to hash password"))?
        .to_string();
    Ok(hash)
}

pub fn verify(password: &str, hash: &str) -> std::io::Result<bool> {
    let argon2 = argon2::Argon2::default();
    let hash = PasswordHash::new(hash)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to parse hash"))?;
    let result = argon2
        .verify_password(password.as_bytes(), &hash)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to verify password"))
        .is_ok();

    Ok(result)
}

// test password
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password() -> std::io::Result<()> {
        let hash = password("password")?;
        assert!(verify("password", &hash)?);
        assert!(!verify("password2", &hash)?);
        Ok(())
    }
}

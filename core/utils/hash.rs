use argon2::{password_hash::SaltString, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

#[no_panic::no_panic]
pub fn password(password: &str) -> std::io::Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = argon2::Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to hash password"))?
        .to_string();
    Ok(hash)
}

#[no_panic::no_panic]
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

#[cfg(test)]
mod tests {
    use super::password;
    use super::verify;

    #[test]
    fn test() -> std::io::Result<()> {
        let some_hash = password("somepw")?;
        assert!(verify("somepw", &some_hash)?);
        Ok(())
    }
}

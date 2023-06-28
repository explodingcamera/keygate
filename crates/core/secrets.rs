use std::{fmt::Debug, sync::RwLock};

pub struct SecretStore {
    storage: RwLock<SecretsInner>,
}

impl Debug for SecretStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretStore").finish()
    }
}

pub struct Ed25519KeyPair(String);

pub struct Secrets {
    pub jwt_ed25519_keypair: Ed25519KeyPair,
}

struct SecretsInner {
    jwt_ed25519_keypair: Ed25519KeyPair,
}

impl SecretStore {
    pub fn new(secrets: Secrets) -> Self {
        Self {
            storage: RwLock::new(SecretsInner {
                jwt_ed25519_keypair: secrets.jwt_ed25519_keypair,
            }),
        }
    }

    // pub fn jwt_ed25519_keypair(&self) -> Result<Ed25519KeyPair, KeygateError> {
    //     Ok(self
    //         .storage
    //         .read()
    //         .map_err(|_| KeygateError::LockPoisoned("secret store".to_string()))?
    //         .jwt_ed25519_keypair
    //         .clone())
    // }
}

use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use dashmap::DashMap;
use keygate_utils::{
    random::secure_random_id,
    tokens::{ed25519, SignatureAlgorithm},
};
use time::OffsetDateTime;

use crate::{
    database::{models, DatabasePool},
    KeygateError, KeygateInternal, KeygateResult,
};

pub enum Keypair {
    Ed25519(ed25519::Ed25519Keypair),
}

#[derive(Clone)]
pub enum PublicKey {
    Ed25519(ed25519::VerifyingKey),
}

#[derive(Clone)]
pub struct PublicKeyData {
    pub node_id: String,
    pub valid_until: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub key: PublicKey,

    pub last_checked: OffsetDateTime,
}

pub struct Secrets {
    keygate: OnceLock<Arc<KeygateInternal>>,
    active_keypairs: DashMap<String, Keypair>,
    public_keys: DashMap<String, PublicKeyData>,
}

impl Debug for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secrets")
            .field("keygate", &self.keygate)
            .field("owned_keys", &format!("{:?} entries", self.active_keypairs.len()))
            .field("public_keys", &format!("{:?} entries", self.public_keys.len()))
            .finish()
    }
}

impl Secrets {
    pub(crate) fn new() -> Self {
        Self {
            keygate: OnceLock::new(),
            active_keypairs: DashMap::new(),
            public_keys: DashMap::new(),
        }
    }

    fn db(&self) -> &DatabasePool {
        &self.keygate.get().expect("Keygate not initialized").db
    }

    pub async fn get_public_key(&self, key_id: &str) -> KeygateResult<Option<PublicKeyData>> {
        match self.public_keys.get(key_id).map(|key| key.value().clone()) {
            Some(key) if key.last_checked >= (OffsetDateTime::now_utc() - time::Duration::minutes(5)) => Ok(Some(key)),
            _ => {
                let key = self.public_key_from_db(key_id).await?;
                if let Some(ref new_key) = key {
                    self.public_keys.insert(key_id.to_string(), new_key.clone());
                }
                Ok(key)
            }
        }
    }

    async fn public_key_from_db(&self, key_id: &str) -> KeygateResult<Option<PublicKeyData>> {
        let key: models::PublicKey =
            sqlx::query_as!(models::PublicKey, r#"SELECT * FROM PublicKey WHERE id = $0"#, key_id)
                .fetch_one(self.db())
                .await?;

        let key = PublicKeyData {
            key: match key.key_type.as_str() {
                "ed25519" => {
                    let key: [u8; 32] = key.public_key[..]
                        .try_into()
                        .map_err(|_| KeygateError::ValidationError("Invalid public key: Invalid Length".into()))?;

                    let key = ed25519::VerifyingKey::from_bytes(&key)
                        .map_err(|_| KeygateError::ValidationError("Invalid public key: Invalid Ed25519 key".into()))?;

                    PublicKey::Ed25519(key)
                }
                key_type => return Err(KeygateError::ValidationError(format!("Invalid key type: {}", key_type))),
            },
            node_id: key.node_id,
            revoked_at: key.revoked_at,
            valid_until: key.valid_until,
            last_checked: OffsetDateTime::now_utc(),
        };

        Ok(Some(key))
    }

    pub fn get_public_keys(&self) -> Vec<PublicKeyData> {
        self.public_keys.iter().map(|key| key.value().clone()).collect()
    }

    pub(crate) fn set_keygate(&self, keygate: Arc<KeygateInternal>) {
        self.keygate.set(keygate).unwrap();
    }

    pub(crate) async fn ensure_keypair(&self) {
        if self.active_keypairs.is_empty() {
            tracing::debug!("No signing keys are configured, generating a new one");
            let new_key_id = self.generate_signing_key();
        }
    }

    fn generate_signing_key(&self) -> String {
        let keypair = ed25519::Ed25519Keypair::generate();
        let public_key = keypair.public_key();
        let key_id = secure_random_id();
        self.active_keypairs.insert(key_id.clone(), Keypair::Ed25519(keypair));
        key_id
    }
}

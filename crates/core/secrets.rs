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

use crate::KeygateInternal;

pub enum Keypair {
    Ed25519(ed25519::Ed25519Keypair),
}

pub enum PublicKey {
    Ed25519(ed25519::VerifyingKey),
}

pub struct PublicKeyData {
    pub node_id: String,
    pub valid_until: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub key: PublicKey,
}

pub struct Secrets {
    keygate: OnceLock<Arc<KeygateInternal>>,
    owned_keys: DashMap<String, Keypair>,
    public_keys: DashMap<String, PublicKeyData>,
}

impl Debug for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secrets")
            .field("keygate", &self.keygate)
            .field("owned_keys", &format!("{:?} entries", self.owned_keys.len()))
            .field("public_keys", &format!("{:?} entries", self.public_keys.len()))
            .finish()
    }
}

impl Secrets {
    pub(crate) fn new() -> Self {
        Self {
            keygate: OnceLock::new(),
            owned_keys: DashMap::new(),
            public_keys: DashMap::new(),
        }
    }

    pub(crate) fn set_keygate(&self, keygate: Arc<KeygateInternal>) {
        self.keygate.set(keygate).unwrap();
    }

    pub(crate) async fn run(&self) {
        if self.owned_keys.is_empty() {
            tracing::warn!("No signing keys are configured, generating a new one");
            let new_key_id = self.generate_signing_key();
        }
    }

    fn generate_signing_key(&self) -> String {
        let keypair = ed25519::Ed25519Keypair::generate();
        let public_key = keypair.public_key();
        let key_id = secure_random_id();
        self.owned_keys.insert(key_id.clone(), Keypair::Ed25519(keypair));
        key_id
    }
}

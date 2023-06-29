#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::cell::OnceCell;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

use config::KeygateConfigInternal;
use prisma::PrismaClient;
pub use proto::models;

mod api;
mod settings;

pub mod config;
mod secrets;
use arc_swap::ArcSwap;
use config::Configuration;
pub use config::Configuration as KeygateConfig;

use settings::KeygateSettings;
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
pub enum Health {
    Healthy,
    Starting,
    Unhealthy,
}

#[derive(Error, Debug)]
pub enum KeygateError {
    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("poisoned lock: {0}")]
    LockPoisoned(String),

    #[error("unknown error")]
    Unknown,
}

pub type KeygateResult<T> = Result<T, KeygateError>;

pub type KeygateSecretsStore = Arc<secrets::SecretStore>;
pub type KeygateSecrets = secrets::Secrets;

#[derive(Debug)]
pub(crate) struct KeygateInternal {
    pub config: KeygateConfigInternal,
    pub prisma: PrismaClient,
    pub health: ArcSwap<Health>,
    pub settings: KeygateSettings,
}

#[derive(Debug, Clone)]
pub struct Keygate {
    inner: Arc<KeygateInternal>,
    // pub identity: Arc<api::Identity>,
}

impl Keygate {
    pub async fn new(config: Configuration) -> Result<Self, KeygateError> {
        let config = Arc::new(RwLock::new(config));
        unimplemented!();
        // Keygate::new_with_storage(config, storage, secrets).await
        // Ok(res.await)
    }

    pub async fn new_with_storage(config: KeygateConfigInternal, prisma: PrismaClient) -> Self {
        let internal = Arc::new(KeygateInternal {
            config,
            prisma,
            health: ArcSwap::from_pointee(Health::Starting),
            settings: KeygateSettings::new(),
        });

        internal.settings.set_keygate(internal.clone());

        Keygate {
            inner: internal,
            // identity: Arc::new(api::Identity::new(internal)),
        }
    }
}

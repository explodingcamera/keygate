#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::sync::Arc;

use config::KeygateConfigInternal;

// mod api;
mod database;
mod settings;

pub mod config;
mod secrets;
use arc_swap::ArcSwap;
use config::Configuration;
pub use config::Configuration as KeygateConfig;

use database::DatabasePool;
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

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

pub type KeygateResult<T> = Result<T, KeygateError>;

pub type KeygateSecretsStore = Arc<secrets::SecretStore>;
pub type KeygateSecrets = secrets::Secrets;

#[derive(Debug)]
pub(crate) struct KeygateInternal {
    pub config: KeygateConfigInternal,
    pub db: DatabasePool,
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
        let config = Arc::new(config);
        sqlx::any::install_default_drivers();
        let db = DatabasePool::connect("sqlite://:memory:").await?;
        sqlx::migrate!().run(&db).await.expect("Failed to run migrations");

        Ok(Keygate::new_with_storage(config, db).await)
    }

    pub async fn new_with_storage(config: KeygateConfigInternal, db: DatabasePool) -> Self {
        let internal = Arc::new(KeygateInternal {
            config,
            db,
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

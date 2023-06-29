use std::sync::{Arc, Mutex, OnceLock};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use keygate_utils::atomic::AtomicDateTime;
use prisma::PrismaClient;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::KeygateInternal;

#[derive(Debug)]
pub struct KeygateSettings {
    keygate: OnceLock<Arc<KeygateInternal>>,
    global: Mutex<Option<GlobalSettings>>,
    global_updated_at: AtomicDateTime,
    applications: DashMap<String, (ApplicationSettings, DateTime<Utc>)>,
}

const UPDATE_INTERVAL: i64 = 60 * 2;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("poisoned lock: {0}")]
    LockPoisoned(String),

    #[error("database error: {0}")]
    DatabaseError(String),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("unknown error")]
    Unknown,
}

impl KeygateSettings {
    pub fn new() -> Self {
        Self {
            keygate: OnceLock::new(),
            global: Mutex::new(None),
            global_updated_at: AtomicDateTime::new(),
            applications: DashMap::new(),
        }
    }

    fn client(&self) -> &PrismaClient {
        &self.keygate.get().expect("Keygate not initialized").prisma
    }

    pub async fn global(&self) -> Result<GlobalSettings, SettingsError> {
        let mut global = self.global.lock().map_err(|e| SettingsError::LockPoisoned(e.to_string()))?;
        if global.is_none() || self.global_updated_at.get().timestamp() < Utc::now().timestamp() - UPDATE_INTERVAL {
            let app = self
                .client()
                .application()
                .find_unique(prisma::application::UniqueWhereParam::IdEquals("global".to_string()))
                .exec()
                .await
                .map_err(|e| SettingsError::DatabaseError(e.to_string()))?
                .ok_or(SettingsError::DatabaseError("Global settings not found".to_string()))?;

            let new_global = serde_json::from_str(&app.setting).map_err(|e| SettingsError::SerializationError(e.to_string()))?;
            self.global_updated_at.set(Utc::now());
            *global = Some(new_global);
        }

        Ok(global.clone().expect("Global settings is none, this should be impossible"))
    }

    pub(crate) fn set_keygate(&self, keygate: Arc<KeygateInternal>) {
        self.keygate.set(keygate).unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSettings {}

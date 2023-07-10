use std::sync::{Arc, Mutex, OnceLock};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use keygate_utils::atomic::AtomicDateTime;
use prisma::PrismaClient;
use proto::models::{ApplicationSettings, GlobalSettings};
use proto::Message;
use thiserror::Error;

use crate::KeygateInternal;

#[derive(Debug)]
pub struct KeygateSettings {
    keygate: OnceLock<Arc<KeygateInternal>>,
    global: Mutex<Option<GlobalSettings>>,
    global_updated_at: AtomicDateTime, // should be a bit more efficient then in the mutex
    applications: DashMap<String, (ApplicationSettings, DateTime<Utc>)>, // seperate dates here would be too much work lol
}

const GLOBAL_SETTINGS_UPDATE_INTERVAL: i64 = 60 * 2;
const APPLICATION_SETTINGS_UPDATE_INTERVAL: i64 = 60 * 2;

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

    pub async fn update_global(&self, settings: GlobalSettings) -> Result<(), SettingsError> {
        let updated_settings = self
            .client()
            .application()
            .update(
                prisma::application::UniqueWhereParam::IdEquals("global".to_string()),
                vec![prisma::application::settings::set(settings.encode_to_vec())],
            )
            .exec()
            .await
            .map_err(|e| SettingsError::DatabaseError(e.to_string()))?;

        self.global_updated_at.set(Utc::now());
        self.global
            .lock()
            .map_err(|e| SettingsError::LockPoisoned(e.to_string()))?
            .replace(GlobalSettings::decode(updated_settings.settings.as_slice()).expect("Invalid global settings"));

        Ok(())
    }

    pub async fn update_app(&self, application_id: &str, settings: ApplicationSettings) -> Result<(), SettingsError> {
        let updated_settings = self
            .client()
            .application()
            .update(
                prisma::application::UniqueWhereParam::IdEquals(application_id.to_string()),
                vec![prisma::application::settings::set(settings.encode_to_vec())],
            )
            .exec()
            .await
            .map_err(|e| SettingsError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn global(&self) -> Result<GlobalSettings, SettingsError> {
        let global = {
            self.global
                .lock()
                .map_err(|e| SettingsError::LockPoisoned(e.to_string()))?
                .clone() // clone so we can drop the lock
        };

        let outdated = self.global_updated_at.get().timestamp() < Utc::now().timestamp() - GLOBAL_SETTINGS_UPDATE_INTERVAL;

        if global.is_none() || outdated {
            let app = self
                .client()
                .application()
                .find_unique(prisma::application::UniqueWhereParam::IdEquals("global".to_string()))
                .exec()
                .await
                .map_err(|e| SettingsError::DatabaseError(e.to_string()))?;

            let new_global = match app {
                Some(app) => {
                    if app.id != "global" {
                        return Err(SettingsError::DatabaseError(format!("Application {} not found", "global")));
                    }
                    GlobalSettings::decode(app.settings.as_slice())
                        .map_err(|e| SettingsError::SerializationError(e.to_string()))?
                }
                None => {
                    let new_global = default_global_settings();
                    let app = prisma::application::Create {
                        id: "global".to_string(),
                        settings: new_global.encode_to_vec(),
                        _params: Default::default(),
                    };

                    app.to_query(self.client())
                        .exec()
                        .await
                        .map_err(|e| SettingsError::DatabaseError(e.to_string()))?;

                    new_global
                }
            };

            self.global_updated_at.set(Utc::now());
            return Ok(new_global);
        }

        Ok(global.clone().expect("Global settings is none, this should be impossible"))
    }

    pub async fn app(&self, application_id: &str) -> Result<Option<ApplicationSettings>, SettingsError> {
        let outdated = self
            .applications
            .get(application_id)
            .map(|d| d.1.timestamp() < Utc::now().timestamp() - APPLICATION_SETTINGS_UPDATE_INTERVAL)
            .unwrap_or(true);

        if outdated {
            let Some(app) = self
                .client()
                .application()
                .find_unique(prisma::application::UniqueWhereParam::IdEquals(application_id.to_string()))
                .exec()
                .await
                .map_err(|e| SettingsError::DatabaseError(e.to_string()))? else {
                    return Ok(None);
                };

            let new_app = ApplicationSettings::decode(app.settings.as_slice())
                .map_err(|e| SettingsError::SerializationError(e.to_string()))?;

            self.applications
                .insert(application_id.to_string(), (new_app.clone(), Utc::now()));

            return Ok(Some(new_app));
        }

        Ok(Some(
            self.applications
                .get(application_id)
                .map(|app| app.0.clone())
                .expect("Application settings not found, this should not be possible"),
        ))
    }

    pub(crate) fn set_keygate(&self, keygate: Arc<KeygateInternal>) {
        self.keygate.set(keygate).unwrap();
    }
}

fn default_global_settings() -> GlobalSettings {
    GlobalSettings {}
}

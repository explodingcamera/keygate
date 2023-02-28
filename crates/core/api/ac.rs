use crate::{KeygateConfigInternal, KeygateError, KeygateStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessControlError {
    #[error("unknown error {0}")]
    Unknown(String),
}

pub struct AccessControl {
    config: KeygateConfigInternal,
    access_control_config: AccessControlConfig,
    storage: KeygateStorage,
}

impl AccessControl {
    pub async fn new(config: KeygateConfigInternal, storage: KeygateStorage) -> Self {
        Self { config, storage }
    }
}

impl AccessControl {}

pub struct AccessControlConfig {
    actor: HashMap<String, Actor>,
    global: Resource,
    resources: HashMap<String, Resource>,
}

pub struct Resource {
    belongs_to: Option<String>,
    permissions: Vec<String>,
    roles: Vec<Role>,
}

pub struct Role {
    name: String,
    permissions: Vec<String>,
    extends: Option<String>,
}

pub struct Actor {
    default_role: Option<String>,
}

pub struct AccessControl {
    actors: Vec<Actor>,
    resources: Vec<Resource>,
    global: Resource,
}

use std::{collections::HashMap, net::IpAddr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Identity {
    pub id: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub emails: HashMap<String, IdentityEmail>,
    pub linked_accounts: Vec<IdentityAccount>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct IdentityEmail {
    pub verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct IdentityAccount {
    pub provider: String,
    pub provider_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub id: String,
    pub ip: Option<IpAddr>,
    pub identity_id: String,
    pub expires_at: u64,
    pub created_at: u64,
}

pub struct IdentitySettings {}

pub struct LoginFlow {
    pub id: String,
    pub identity_id: String,
    pub device_id: String,
    pub expires_at: u64,
    pub created_at: u64,
}

pub struct SignupFlow {
    pub id: String,
    pub identity_id: String,
    pub expires_at: u64,
    pub created_at: u64,
}

pub struct VerificationFlow {
    pub id: String,
    pub identity_id: String,
    pub expires_at: u64,
    pub created_at: u64,
}

pub struct RecoveryFlow {
    pub id: String,
    pub identity_id: String,
    pub expires_at: u64,
    pub created_at: u64,
}

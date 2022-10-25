use std::{collections::HashMap, net::IpAddr};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
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
    pub verified_at: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct IdentityAccount {
    pub provider: String,
    pub provider_id: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Session {
    pub id: String,
    pub ip: Option<IpAddr>,
    pub identity_id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub current_refresh_token: String,
    pub revoked: bool,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct RefreshToken {
    pub id: String,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub expires_at: u64,
    pub created_at: u64,

    pub session_id: String,
    pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct AccessToken {
    pub id: String,
    pub identity_id: String,
    pub refresh_token_id: String,
    pub revoked_at: Option<u64>,
    pub expires_at: u64,
    pub created_at: u64,
}

pub enum Flows {
    EmailLoginFlow,
    EmailSignupFlow,
    VerificationFlow,
    RecoveryFlow,
}

pub struct Flow {
    pub id: String,
    pub flow: Flows,
    pub expires_at: u64,
    pub created_at: u64,
}

pub struct EmailLoginFlow {
    pub email: Option<String>,
    pub identity_id: String,
    pub device_id: String,
}

pub struct EmailSignupFlow {
    pub email: Option<String>,
    pub device_id: String,
}

pub struct VerificationFlow {
    pub device_id: String,
    pub identity_id: String,
    pub verification_token: String,
}

pub struct RecoveryFlow {
    pub identity_id: String,
}

pub struct FlowToken {
    pub id: String,
    pub flow_id: String,
    pub token: String,
    pub expires_at: u64, // has to be before the flow expires
    pub created_at: u64,
}

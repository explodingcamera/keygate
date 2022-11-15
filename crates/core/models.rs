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
    pub password_hash: Option<String>,
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

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Session {
    pub id: String,
    pub ip: Option<IpAddr>,
    pub identity_id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub current_refresh_token: String,
    pub revoked_at: Option<u64>,
}

// refresh tokens are stored in a separate table, since - while only one refresh token is valid at a time - we want
// to keep a history of all refresh tokens issued to a session so we can detect if a refresh token has been stolen
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct RefreshToken {
    pub id: String,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub expires_at: u64,
    pub created_at: u64,
    pub revoked_at: Option<u64>,

    pub session_id: String,
    pub identity_id: String,
    pub access_token_id: String,
}

// If a user is authenticated in multiple tabs on the same device, they will have multiple access tokens at the same time.
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct AccessToken {
    pub id: String,
    pub identity_id: String,
    pub refresh_token_id: String,
    pub revoked_at: Option<u64>,
    pub expires_at: u64,
    pub created_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub enum Process {
    UsernameEmailLogin(BaseProcess<UsernameEmailLoginProcess>),
    UsernameEmailSignup(BaseProcess<UsernameEmailSignupProcess>),
    Verification(BaseProcess<VerificationProcess>),
    Recovery(BaseProcess<RecoveryProcess>),
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct BaseProcess<T> {
    pub id: String,
    pub process: T,
    pub expires_at: u64,
    pub created_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct UsernameEmailLoginProcess {
    pub identity_id: String,
    pub device_id: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct UsernameEmailSignupProcess {
    pub device_id: String,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct VerificationProcess {
    pub identity_id: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct RecoveryProcess {
    pub identity_id: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct ProcessToken {
    pub id: String,
    pub process_id: String,
    pub expires_at: u64, // has to be before the process expires
    pub created_at: u64,
}

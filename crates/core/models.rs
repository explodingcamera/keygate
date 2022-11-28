use std::{collections::HashMap, net::IpAddr};

use serde::{Deserialize, Serialize};

type ProviderID = String;
type Email = String;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Identity {
    pub id: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub emails: HashMap<Email, IdentityEmail>,
    pub linked_accounts: HashMap<ProviderID, IdentityAccount>,
    pub password_hash: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct IdentityEmail {
    pub verified: bool,
    pub verified_at: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct IdentityAccount {
    pub provider_id: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Session {
    pub id: String,
    pub ip: Option<IpAddr>,
    pub identity_id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub current_refresh_token: String,
    pub revoked_at: Option<i64>,
}

// refresh tokens are stored in a separate table, since - while only one refresh token is valid at a time - we want
// to keep a history of all refresh tokens issued to a session so we can detect if a refresh token has been stolen
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct RefreshToken {
    pub id: String,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub expires_at: i64,
    pub created_at: i64,
    pub revoked_at: Option<i64>,

    pub session_id: String,
    pub identity_id: String,
    pub access_token_id: String,
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
    pub completed_at: Option<u64>,
    pub expires_at: i64,
    pub created_at: i64,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct UsernameEmailLoginProcess {
    pub identity_id: String,
    pub device_id: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct UsernameEmailSignupProcess {
    pub device_id: String,
    pub email: Option<(String, IdentityEmail)>,
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
    pub expires_at: i64, // has to be before the process expires
    pub created_at: i64,
}

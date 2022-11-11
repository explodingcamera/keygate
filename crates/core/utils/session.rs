use chrono::{DateTime, Utc};

use crate::models;

use super::tokens::{generate_access_token_id, generate_refresh_token_id};

pub struct Rotation {
    pub old_refresh_token: models::RefreshToken,
    pub new_refresh_token: models::RefreshToken,
    pub new_access_token: models::AccessToken,
    pub updated_session: models::Session,
}

pub fn rotate_refresh_token(
    old_refresh_token: models::RefreshToken,
    session: models::Session,

    refresh_expires_at: DateTime<Utc>,
    access_expires_at: DateTime<Utc>,
) -> Rotation {
    if session.revoked_at.is_some() {
        panic!("Cannot rotate a revoked session, this is a bug that should never happen");
    }

    let new_refresh_token_id = generate_refresh_token_id();
    let new_access_token_id = generate_access_token_id();

    let mut old_refresh_token = old_refresh_token;
    old_refresh_token.next = Some(new_refresh_token_id.clone());

    let new_refresh_token = models::RefreshToken {
        prev: Some(old_refresh_token.id.clone()),
        next: None,
        session_id: old_refresh_token.session_id.clone(),
        access_token_id: new_access_token_id.clone(),
        id: new_refresh_token_id.clone(),
        identity_id: old_refresh_token.identity_id.clone(),
        created_at: chrono::Utc::now().timestamp().unsigned_abs(),
        expires_at: refresh_expires_at.timestamp().unsigned_abs(),
        revoked_at: None,
    };

    let new_access_token = models::AccessToken {
        id: new_access_token_id,
        identity_id: old_refresh_token.identity_id.clone(),
        refresh_token_id: new_refresh_token_id.clone(),
        created_at: chrono::Utc::now().timestamp().unsigned_abs(),
        expires_at: access_expires_at.timestamp().unsigned_abs(),
        revoked_at: None,
    };

    let mut updated_session = session;
    updated_session.current_refresh_token = new_refresh_token_id;

    Rotation {
        old_refresh_token,
        new_refresh_token,
        new_access_token,
        updated_session,
    }
}

use std::sync::Arc;

use keygate_utils::{
    random::secure_random_id,
    tokens::{AccessToken, RawAccessToken, RawRefreshToken, RefreshToken},
};

use crate::{database::DatabasePool, KeygateInternal};

use super::APIError;

#[derive(Debug, Clone)]
pub struct Session {
    keygate: Arc<KeygateInternal>,
}

// pub static KEYPAIR: KeygateKeypair = KeygateKeypair::generate(keygate_utils::tokens::Algorithm::Ed25519);

impl Session {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn db(&self) -> &DatabasePool {
        &self.keygate.db
    }

    pub async fn create(&self, identity_id: String) -> Result<(RawRefreshToken, RawAccessToken), APIError> {
        let session_id = secure_random_id();
        let exp = time::OffsetDateTime::now_utc() + time::Duration::minutes(15);
        // let refresh_token = KEYPAIR;

        // sqlx::query!(
        //     r#"
        //     INSERT INTO Sessions (

        //     )
        //     VALUES (?, ?, ?, ?, ?, ?)
        // "#,
        // )
        // .execute(self.db())
        // .await?;

        unimplemented!()
    }

    async fn rotate_refresh(
        &self,
        token: keygate_utils::tokens::RefreshToken,
    ) -> Result<keygate_utils::tokens::AccessToken, APIError> {
        unimplemented!()
    }

    async fn validate_access_token(&self, something: ()) -> Result<(), APIError> {
        unimplemented!()
    }

    async fn check_revoked(&self, something: ()) -> Result<(), APIError> {
        unimplemented!()
    }
}

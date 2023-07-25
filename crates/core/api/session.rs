use std::sync::Arc;

use crate::{database::DatabasePool, KeygateInternal};

use super::APIError;

#[derive(Debug, Clone)]
pub struct Session {
    keygate: Arc<KeygateInternal>,
}

impl Session {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn db(&self) -> &DatabasePool {
        &self.keygate.db
    }

    async fn create(&self, something: ()) -> Result<(), APIError> {
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

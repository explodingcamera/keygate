use std::sync::Arc;

use super::APIError;
use crate::{database::DatabasePool, KeygateInternal};

#[derive(Debug, Clone)]
pub struct Template {
    keygate: Arc<KeygateInternal>,
}

impl Template {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn db(&self) -> &DatabasePool {
        &self.keygate.db
    }

    async fn get(&self, something: ()) -> Result<(), APIError> {
        unimplemented!()
    }
}

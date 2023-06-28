use std::sync::Arc;

use crate::KeygateInternal;
use prisma::PrismaClient;
use proto::models;

use super::{APIError, UserIdentifier};

#[derive(Debug, Clone)]
pub struct Template {
    keygate: Arc<KeygateInternal>,
}

impl Template {
    pub(crate) fn new(keygate: Arc<KeygateInternal>) -> Self {
        Self { keygate }
    }

    fn client(&self) -> &PrismaClient {
        &self.keygate.prisma
    }

    async fn get(&self, something: ()) -> Result<(), APIError> {
        unimplemented!()
    }
}

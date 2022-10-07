use crate::{Keygate, KeygateError};

pub trait Recovery: Send + Sync {
    /// Initiate a recovery flow for a user.
    fn recovery_initiate(&self) -> Result<(), KeygateError>;

    /// Complete a recovery flow for a user.
    fn recovery_complete(&self) -> Result<(), KeygateError>;

    /// Get a recovery flow
    fn recovery(&self) -> Result<(), KeygateError>;
}

impl Recovery for Keygate {
    fn recovery_initiate(&self) -> Result<(), KeygateError> {
        todo!()
    }

    fn recovery_complete(&self) -> Result<(), KeygateError> {
        todo!()
    }

    fn recovery(&self) -> Result<(), KeygateError> {
        todo!()
    }
}

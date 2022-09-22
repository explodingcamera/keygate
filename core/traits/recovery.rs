use crate::{KeySignal, KeysignalError};

pub trait Recovery: Send + Sync {
    /// Initiate a recovery flow for a user.
    fn recovery_initiate(&self) -> Result<(), KeysignalError>;

    /// Complete a recovery flow for a user.
    fn recovery_complete(&self) -> Result<(), KeysignalError>;

    /// Get a recovery flow
    fn recovery(&self) -> Result<(), KeysignalError>;
}

impl Recovery for KeySignal {
    fn recovery_initiate(&self) -> Result<(), KeysignalError> {
        todo!()
    }

    fn recovery_complete(&self) -> Result<(), KeysignalError> {
        todo!()
    }

    fn recovery(&self) -> Result<(), KeysignalError> {
        todo!()
    }
}

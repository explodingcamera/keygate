use crate::{KeySignal, KeysignalError};

pub trait Session: Send + Sync {
    fn session_validate(&self) -> Result<(), KeysignalError>;
    fn session_invalidate(&self) -> Result<(), KeysignalError>;
    fn session_refresh(&self) -> Result<(), KeysignalError>;
    fn sessions(&self) -> Result<(), KeysignalError>;
}

impl Session for KeySignal {
    fn sessions(&self) -> Result<(), KeysignalError> {
        todo!()
    }

    fn session_validate(&self) -> Result<(), KeysignalError> {
        todo!()
    }

    fn session_invalidate(&self) -> Result<(), KeysignalError> {
        todo!()
    }

    fn session_refresh(&self) -> Result<(), KeysignalError> {
        todo!()
    }
}

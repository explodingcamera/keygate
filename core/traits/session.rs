use crate::{Keygate, KeygateError};

pub trait Session: Send + Sync {
    fn session_validate(&self) -> Result<(), KeygateError>;
    fn session_invalidate(&self) -> Result<(), KeygateError>;
    fn session_refresh(&self) -> Result<(), KeygateError>;
    fn sessions(&self) -> Result<(), KeygateError>;
}

impl Session for Keygate {
    fn sessions(&self) -> Result<(), KeygateError> {
        todo!()
    }

    fn session_validate(&self) -> Result<(), KeygateError> {
        todo!()
    }

    fn session_invalidate(&self) -> Result<(), KeygateError> {
        todo!()
    }

    fn session_refresh(&self) -> Result<(), KeygateError> {
        todo!()
    }
}

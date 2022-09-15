use crate::{storage::Storage, KeySignal};

pub trait Session: Send + Sync {
    fn session_validate(&self) -> Result<(), String>;
    fn session_invalidate(&self) -> Result<(), String>;
    fn session_refresh(&self) -> Result<(), String>;
    fn sessions(&self) -> Result<(), String>;
}

impl Session for KeySignal {
    fn sessions(&self) -> Result<(), String> {
        todo!()
    }

    fn session_validate(&self) -> Result<(), String> {
        todo!()
    }

    fn session_invalidate(&self) -> Result<(), String> {
        todo!()
    }

    fn session_refresh(&self) -> Result<(), String> {
        todo!()
    }
}

use crate::KeySignal;

pub trait Recovery: Send + Sync {
    /// Initiate a recovery flow for a user.
    fn recovery_initiate(&self) -> Result<(), String>;

    /// Complete a recovery flow for a user.
    fn recovery_complete(&self) -> Result<(), String>;

    /// Get a recovery flow
    fn recovery(&self) -> Result<(), String>;
}

impl Recovery for KeySignal {
    fn recovery_initiate(&self) -> Result<(), String> {
        todo!()
    }

    fn recovery_complete(&self) -> Result<(), String> {
        todo!()
    }

    fn recovery(&self) -> Result<(), String> {
        todo!()
    }
}

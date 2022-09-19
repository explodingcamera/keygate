use crate::KeySignal;

pub trait Identity: Send + Sync {
    fn identity(&self) -> Result<(), String>;
    fn identity_delete(&self) -> Result<(), String>;
    fn identity_update(&self) -> Result<(), String>;
    fn identities(&self) -> Result<(), String>;
}

impl Identity for KeySignal {
    fn identity(&self) -> Result<(), String> {
        todo!()
    }

    fn identity_delete(&self) -> Result<(), String> {
        todo!()
    }

    fn identity_update(&self) -> Result<(), String> {
        todo!()
    }

    fn identities(&self) -> Result<(), String> {
        todo!()
    }
}

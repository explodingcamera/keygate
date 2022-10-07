use crate::Keygate;

pub trait Registration: Send + Sync {}
impl Registration for Keygate {}

use crate::KeySignal;

pub trait Registration: Send + Sync {}
impl Registration for KeySignal {}

use crate::KeySignal;

pub trait Verification: Send + Sync {}
impl Verification for KeySignal {}

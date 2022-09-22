use crate::KeySignal;

pub trait Signup: Send + Sync {}
impl Signup for KeySignal {}

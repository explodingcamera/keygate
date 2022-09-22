use crate::KeySignal;

pub trait OAuth: Send + Sync {}

impl OAuth for KeySignal {}

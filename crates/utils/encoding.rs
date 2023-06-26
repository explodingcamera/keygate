use std::any::Any;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("deserialization error: {0}")]
    DeserializationError(String),
}

pub fn to_bytes(value: &impl Any) -> Result<Vec<u8>, EncodingError> {
    unimplemented!()
}

pub fn from_bytes<T>(bytes: &[u8]) -> Result<T, EncodingError> {
    unimplemented!()
}

#[no_panic::no_panic]
pub fn to_bytes(value: &impl serde::Serialize) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    rmp_serde::to_vec(value)
}

#[no_panic::no_panic]
pub fn from_bytes<T>(bytes: &[u8]) -> Result<T, rmp_serde::decode::Error>
where
    T: serde::de::DeserializeOwned,
{
    rmp_serde::from_slice(bytes)
}

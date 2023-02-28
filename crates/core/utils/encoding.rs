pub fn to_bytes(value: &impl serde::Serialize) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    rmp_serde::to_vec(value)
}

pub fn from_bytes<T>(bytes: &[u8]) -> Result<T, rmp_serde::decode::Error>
where
    T: serde::de::DeserializeOwned,
{
    rmp_serde::from_slice(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() -> Result<(), rmp_serde::encode::Error> {
        let bytes = to_bytes(&"hello")?;
        assert_eq!(bytes, vec![0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
        Ok(())
    }

    #[test]
    fn test_from_bytes() -> Result<(), rmp_serde::decode::Error> {
        let bytes = vec![0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let value: String = from_bytes(&bytes)?;
        assert_eq!(value, "hello");
        Ok(())
    }
}

use sha1::{Digest, Sha1};

pub fn hash(str: &str) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(str);
    hasher.finalize().into()
}

// tests for the hash function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let hash = hash("test");
        assert_eq!(
            hash,
            [0xa9, 0x4a, 0x8f, 0xe5, 0xcc, 0xb1, 0x9b, 0xa6, 0x1c, 0x4c, 0x08, 0x73, 0xd3, 0x91, 0xe9, 0x87, 0x98, 0x2f, 0xbb, 0xd3]
        );
    }
}

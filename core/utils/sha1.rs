use sha1::{Digest, Sha1};

pub fn hash(str: &str) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(str);
    hasher.finalize().into()
}

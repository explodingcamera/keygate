use rand_core::{OsRng, RngCore};

#[inline]
pub fn random(size: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; size];
    OsRng.fill_bytes(&mut result[..]);
    result
}

pub const RANDOMID_ALPHABET: [char; 64] = [
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// based on nanoid-rs - MIT License Copyright (c) 2017 Nikolay Govorov
pub fn randomid(size: usize) -> String {
    let mask = RANDOMID_ALPHABET.len().next_power_of_two() - 1;
    let step: usize = 8 * size / 5;
    // Assert that the masking does not truncate the alphabet. (See #9)
    debug_assert!(RANDOMID_ALPHABET.len() <= mask + 1);
    let mut id = String::with_capacity(size);
    loop {
        let bytes = random(step);
        for &byte in &bytes {
            let byte = byte as usize & mask;
            if RANDOMID_ALPHABET.len() > byte {
                id.push(RANDOMID_ALPHABET[byte]);
                if id.len() == size {
                    return id;
                }
            }
        }
    }
}

pub fn secure_random_id() -> String {
    randomid(21)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_random_id() {
        let id = secure_random_id();
        assert_eq!(id.len(), 21);
    }
}

use nanoid::nanoid;

pub fn secure_random_id() -> String {
    nanoid!(21)
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

pub fn is_valid_device_id(device_id: &str) -> bool {
    device_id.len() == 16
}

pub fn is_valid_id(random_string: &str) -> bool {
    random_string.len() == 21
        && random_string
            .chars()
            .all(|c| nanoid::alphabet::SAFE.contains(&c))
}

pub fn is_valid_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    re.is_match(email)
}

pub fn is_valid_username(username: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+$").unwrap();
    re.is_match(username) && username.len() >= 3 && username.len() <= 32
}

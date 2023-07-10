use base64::Engine;

pub trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for [u8] {
    fn to_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(self)
    }
}

impl ToBase64 for Vec<u8> {
    fn to_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(self)
    }
}

impl ToBase64 for String {
    fn to_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(self.as_bytes())
    }
}

impl ToBase64 for &str {
    fn to_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(self.as_bytes())
    }
}

pub trait FromBase64 {
    fn decode_base64(&self) -> Result<Vec<u8>, base64::DecodeError>;
    fn decode_base64_string(&self) -> Result<String, base64::DecodeError> {
        String::from_utf8(self.decode_base64()?).map_err(|_| base64::DecodeError::InvalidByte(0, 0))
    }
}

impl FromBase64 for String {
    fn decode_base64(&self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::engine::general_purpose::STANDARD_NO_PAD.decode(self.as_bytes())
    }
}

impl FromBase64 for &str {
    fn decode_base64(&self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::engine::general_purpose::STANDARD_NO_PAD.decode(self.as_bytes())
    }
}

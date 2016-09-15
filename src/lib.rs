pub struct UrlEncoder {}

impl UrlEncoder {
    pub fn new() -> UrlEncoder {
        UrlEncoder {}
    }

    pub fn encode(&self, data: &str) -> String {
        let mut escaped = String::new();
        for b in data.as_bytes().iter() {
            match *b as char {
                // Accepted characters
                'A'...'Z' | 'a'...'z' | '-' | '_' | '.' | '~' => escaped.push(*b as char),

                // Everything else is percent-encoded
                b => escaped.push_str(format!("%{:02X}", b as u32).as_str()),
            };
        }
        return escaped;
    }
}

pub struct UrlDecoder {}

impl UrlDecoder {
    pub fn new() -> UrlDecoder {
        UrlDecoder {}
    }

    pub fn decode(&self, data: &str) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::UrlEncoder;

    #[test]
    fn it_encodes_successfully() {
        let expected = "this%20that";

        let encoder = UrlEncoder::new();
        assert_eq!(expected, encoder.encode("this that"));
    }
}

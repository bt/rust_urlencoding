use std::str;
use std::string::FromUtf8Error;

pub fn encode(data: &str) -> String {
    let mut escaped = String::new();
    for b in data.as_bytes().iter() {
        match *b as char {
            // Accepted characters
            'A'...'Z' | 'a'...'z' | '0'...'9' | '-' | '_' | '.' | '~' => escaped.push(*b as char),

            // Everything else is percent-encoded
            b => escaped.push_str(format!("%{:02X}", b as u32).as_str()),
        };
    }
    return escaped;
}

pub fn decode(data: &str) -> Result<String, FromUrlEncodingError> {
    let mut unescaped_bytes: Vec<u8> = Vec::new();
    let mut bytes = data.bytes();
    let mut bad_input: Option<u8> = None;
    let mut bytes_to_decode: [u8; 2] = [0, 0];
    let mut byte_being_read = 0;
    while let Some(b) = bytes.next() {
        match b as char {
            'A'...'Z' | 'a'...'z' | '0'...'9' | '-' | '_' | '.' | '~' => unescaped_bytes.push(b),
            '%' => {
                while byte_being_read < 2 {
                    if let Some(b) = bytes.next() {
                        bytes_to_decode[byte_being_read] = b;
                        byte_being_read += 1;
                    } else {
                        return Err(FromUrlEncodingError::UriCharacterError {
                            byte: b,
                        })
                    }
                }
                let hex_str = str::from_utf8(&bytes_to_decode).unwrap();
                if byte_being_read < 2 {
                    bad_input = Some(b);
                    break;
                }
                unescaped_bytes.push(u8::from_str_radix(hex_str, 16).unwrap());
                byte_being_read = 0;
            },
            _ => {
                bad_input = Some(b);
                break;
            }
        }
    }
    match bad_input {
        Some(bad_byte) =>
            Err(FromUrlEncodingError::UriCharacterError {
                byte: bad_byte,
            }),
        None =>
            match String::from_utf8(unescaped_bytes) {
                Err(e) =>
                    Err(FromUrlEncodingError::Utf8CharacterError {
                        error: e,
                    }),
                Ok(s) => Ok(s),
            },
   }
}

#[derive(Debug)]
pub enum FromUrlEncodingError {
    UriCharacterError { byte: u8 },
    Utf8CharacterError { error: FromUtf8Error },
}

#[cfg(test)]
mod tests {
    use super::encode;
    use super::decode;
    use super::FromUrlEncodingError;

    #[test]
    fn it_encodes_successfully() {
        let expected = "this%20that";
        assert_eq!(expected, encode("this that"));
    }

    #[test]
    fn it_encodes_successfully_emoji() {
        let emoji_string = "👾 Exterminate!";
        let expected = "%F0%9F%91%BE%20Exterminate%21";
        assert_eq!(expected, encode(emoji_string));
    }

    #[test]
    fn it_decodes_successfully() {
        let expected = String::from("this that");
        let encoded = "this%20that";
        assert_eq!(expected, decode(encoded).unwrap());
    }

    #[test]
    fn it_decodes_successfully_emoji() {
        let expected = String::from("👾 Exterminate!");
        let encoded = "%F0%9F%91%BE%20Exterminate%21";
        assert_eq!(expected, decode(encoded).unwrap());
    }

    #[test]
    fn it_decodes_unsuccessfully_emoji() {
        let bad_encoded_string = "👾 Exterminate!";
        let expected: u8 = 0xF0;

        match decode(bad_encoded_string).unwrap_err() {
            FromUrlEncodingError::UriCharacterError { byte: b } =>
                assert_eq!(expected, b),
            _ => panic!()
        }
    }
}

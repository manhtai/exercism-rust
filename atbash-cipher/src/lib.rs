/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut count = 1;
    plain.chars().map(|c| { c.to_ascii_lowercase() }).map(|c_lower| {
        let sep = if count % 5 == 0 { " " } else { "" };
        return if c_lower >= 'a' && c_lower <= 'z' {
            count += 1;
            char::from(b'z' - c_lower as u8 + b'a').to_string() + sep
        } else if c_lower >= '0' && c_lower <= '9' {
            count += 1;
            c_lower.to_string() + sep
        } else {
            "".to_owned()
        };
    }).collect::<String>().trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().map(|c| { c.to_ascii_lowercase() }).map(|c_lower| {
        return if c_lower >= 'a' && c_lower <= 'z' {
            char::from(b'a' + b'z' - c_lower as u8).to_string()
        } else if c_lower > '0' && c_lower <= '9' {
            c_lower.to_string()
        } else {
            "".to_owned()
        };
    }).collect()
}

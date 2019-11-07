fn translate(c: char) -> Option<char> {
    let c_lower = c.to_ascii_lowercase();
    return if c_lower >= 'a' && c_lower <= 'z' {
        Some(char::from(b'a' + b'z' - c_lower as u8))
    } else if c_lower >= '0' && c_lower <= '9' {
        Some(c_lower)
    } else {
        None
    };
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars()
        .filter_map(translate)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(translate).collect()
}

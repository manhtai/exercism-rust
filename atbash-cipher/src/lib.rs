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
        .enumerate()
        .flat_map(|(i, item)| if i > 0 && i % 5 == 0 { Some(' ') } else { None }.into_iter().chain(std::iter::once(item)))
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(translate).collect()
}

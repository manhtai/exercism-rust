fn translate(c: char) -> String {
    let c_lower = c.to_ascii_lowercase();
    return if c_lower >= 'a' && c_lower <= 'z' {
        char::from(b'a' + b'z' - c_lower as u8).to_string()
    } else if c_lower >= '0' && c_lower <= '9' {
        c_lower.to_string()
    } else {
        "".to_owned()
    };
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars()
        .map(translate)
        .filter(|c| c != "")
        .collect::<Vec<String>>()
        .chunks(5)
        .map(|c| c.join(""))
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().map(translate).collect()
}

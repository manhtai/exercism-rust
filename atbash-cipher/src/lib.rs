const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const CIPHER: &str = "zyxwvutsrqponmlkjihgfedcba";
const a: u32 = 'a' as u32;
const z: u32 = 'z' as u32;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut cipher = "".to_owned();
    let mut count = 0;
    for c in plain.chars() {
        if count == 5 {
            cipher.push_str(" ");
            count = 0;
        }

        let c_lower = c.to_ascii_lowercase();
        if c_lower >= 'a' && c_lower <= 'z' {
            let range = (c_lower as u32 - a) as usize;
            cipher.push_str(&CIPHER[range..range + 1]);
            count += 1;
        } else if c_lower >= '0' && c_lower <= '9' {
            cipher.push_str(&c_lower.to_string());
            count += 1;
        }
    }
    cipher.trim().to_owned()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut plain = "".to_owned();
    for c in cipher.chars() {
        let c_lower = c.to_ascii_lowercase();
        if c_lower >= 'a' && c_lower <= 'z' {
            let range = (z - c_lower as u32) as usize;
            plain.push_str(&ALPHABET[range..range + 1]);
        } else if c_lower > '0' && c_lower <= '9' {
            plain.push_str(&c_lower.to_string());
        }
    }
    plain
}

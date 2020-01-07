/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let clean_chars = code
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>();

    if clean_chars.len() < 2 || clean_chars.chars().any(|c| !c.is_numeric()) {
        return false;
    }

    let sum = clean_chars
        .chars()
        .rev()
        .map(|c| { c.to_string().parse::<i32>().unwrap() })
        .enumerate()
        .map(|(i, n)|
            if i % 2 == 1 {
                if n * 2 > 9 {
                    n * 2 - 9
                } else {
                    n * 2
                }
            } else {
                n
            })
        .fold(0, |a, b| a + b);

    return sum % 10 == 0;
}

use std::collections::HashSet;

fn sort_word(word: &str) -> String {
    let mut new_word = word
        .to_lowercase()
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    new_word.sort();
    return new_word.iter().collect();
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort_word(word);
    return possible_anagrams.iter()
        .filter(|c| c.to_lowercase() != lowercase_word && sort_word(c) == sorted_word)
        .cloned()
        .collect();
}

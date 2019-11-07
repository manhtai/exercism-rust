use std::collections::HashSet;

fn sort_word(word: &str) -> String {
    let mut new_word = word.chars().map(|c| c.to_lowercase().to_string()).collect::<Vec<String>>();
    new_word.sort();
    return new_word.join("");
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase().to_string();
    let sorted_word = sort_word(word);
    return possible_anagrams.iter()
        .filter(|c| c.to_lowercase() != lowercase_word)
        .filter_map(|c| if sort_word(c) == sorted_word { Some(c.to_owned()) } else { None })
        .collect();
}

use std::collections::HashSet;

fn sort_word(word: &String) -> Vec<char> {
    let mut new_word = word
        .chars()
        .collect::<Vec<char>>();
    new_word.sort();
    new_word
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort_word(&lowercase_word);
    possible_anagrams.iter()
        .filter(|c| {
            let c_lower = c.to_lowercase();
            c_lower != lowercase_word && sort_word(&c_lower) == sorted_word
        })
        .cloned()
        .collect()
}

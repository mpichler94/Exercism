use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_counts = count_chars(word);

    let mut result = HashSet::new();
    for anagram in possible_anagrams {
        if word.to_lowercase() == anagram.to_lowercase() {
            continue;
        }

        let anagram_counts = count_chars(anagram);
        if word_counts == anagram_counts {
            result.insert(*anagram);
        }
    }

    result
}

fn count_chars(word: &str) -> HashMap<char, i32> {
    word.to_lowercase().chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}

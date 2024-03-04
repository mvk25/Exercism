use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let nword = normalize(word);
    let mut nword_sorted = nword.clone();
    nword_sorted.sort_unstable();
    candidates.iter()
        .filter(|c| is_anagram(&nword, &normalize(c), &nword_sorted))
        .copied()
        .collect()
}

fn is_anagram(word1: &[char], word2: &[char], word1_sorted: &[char]) -> bool {
    let mut result = false;
    if word1.len() == word2.len() && word1.ne(word2) {
        let mut list2 = word2.to_owned();
        list2.sort_unstable();
        result = word1_sorted.eq(&list2);
    }
    result
}

fn normalize(word: &str) -> Vec<char> {
    word.to_lowercase().chars().collect()
}
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&str> = HashSet::new();
    for anagram in possible_anagrams {
        println!("1. {} {}", word, anagram);
        let word = word.to_lowercase();
        let copy_lower = anagram.to_lowercase();
        let mut chars: Vec<char> = word.chars().collect();
        let mut anas: Vec<char> = copy_lower.chars().collect();
        chars.sort_unstable();
        anas.sort_unstable();
        let x = &chars.iter().collect::<String>();
        let y = &anas.iter().collect::<String>();
        if x.len() != y.len() {continue;}
        println!("2. {} {}", x, y);
        if x == y {
            if word == copy_lower {continue;}
            res.insert(anagram);
        };

    }

    return res;
}

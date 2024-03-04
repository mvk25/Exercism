use std::collections::HashSet;

fn compare_strings_ignore_order(s1: &str, s2: &str) -> bool {
    // Function to tokenize a string into words and create a HashSet<&str>
    fn tokenize_and_unique_words(s: &str) -> HashSet<&str> {
        s.split_whitespace().collect()
    }

    // Tokenize and create HashSet for each string
    let set1: HashSet<&str> = tokenize_and_unique_words(s1);
    let set2: HashSet<&str> = tokenize_and_unique_words(s2);

    // Compare the sets
    set1 == set2
}

fn main() {
    let string1 = "diaper";
    let string2 = "redipa";

    if compare_strings_ignore_order(string1, string2) {
        println!("The strings contain the same words regardless of order.");
    } else {
        println!("The strings do not contain the same words regardless of order.");
    }
}

use std::collections::{HashMap, HashSet};

fn first_duplicate(values: &[i32]) -> Option<i32> {
    // Return the first value encountered for the second time.
    // Return None when every value is unique.
    let mut unique = HashSet::new();
    for &value in values {
        if !unique.insert(value) {
            return Some(value);
        }
    }
    None
}

fn word_frequencies(words: &[&str]) -> HashMap<String, usize> {
    // Count each word and return owned String keys.
    let mut word_counts = HashMap::new();
    for &word in words {
        *word_counts.entry(word.to_owned()).or_insert(0) += 1;
    }
    word_counts
}

fn main() {
    let values = [4, 7, 2, 7, 9, 2];
    let words = ["rust", "go", "rust", "sql"];

    println!("first duplicate: {:?}", first_duplicate(&values));
    println!("word frequencies: {:?}", word_frequencies(&words));
}

#[cfg(test)]
mod tests {
    use super::{first_duplicate, word_frequencies};
    use std::collections::HashMap;

    #[test]
    fn returns_first_duplicate_in_encounter_order() {
        assert_eq!(first_duplicate(&[4, 7, 2, 7, 9, 2]), Some(7));
    }

    #[test]
    fn returns_none_when_values_are_unique() {
        assert_eq!(first_duplicate(&[4, 7, 2, 9]), None);
    }

    #[test]
    fn counts_words() {
        let actual = word_frequencies(&["rust", "go", "rust", "sql", "go", "rust"]);
        let expected = HashMap::from([
            (String::from("rust"), 3),
            (String::from("go"), 2),
            (String::from("sql"), 1),
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn returns_empty_map_for_empty_input() {
        assert_eq!(word_frequencies(&[]), HashMap::new());
    }
}

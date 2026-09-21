use std::collections::{HashMap, HashSet};

fn first_duplicate(values: &[i32]) -> Option<i32> {
    let _ = HashSet::<i32>::new();
    todo!()
}

fn word_frequencies(words: &[&str]) -> HashMap<String, usize> {
    let _ = HashMap::<String, usize>::new();
    todo!()
}

fn main() {
    println!("{:?}", first_duplicate(&[4, 7, 2, 7]));
    println!("{:?}", word_frequencies(&["rust", "go", "rust"]));
}

#[cfg(test)]
mod tests {
    use super::{first_duplicate, word_frequencies};
    use std::collections::HashMap;

    #[test]
    fn expected_behavior() {
        assert_eq!(first_duplicate(&[4, 7, 2, 7, 9]), Some(7));
        assert_eq!(first_duplicate(&[4, 7, 2, 9]), None);
        assert_eq!(
            word_frequencies(&["rust", "go", "rust"]),
            HashMap::from([(String::from("rust"), 2), (String::from("go"), 1)])
        );
    }
}

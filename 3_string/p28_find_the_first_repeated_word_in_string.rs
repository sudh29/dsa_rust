use std::collections::HashSet;

pub fn first_repeat_word(sentence: &str) -> Option<String> {
    let mut seen = HashSet::new();
    for word in sentence.split_whitespace() {
        if seen.contains(word) {
            return Some(word.to_string());
        }
        seen.insert(word);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_repeat() {
        assert_eq!(
            first_repeat_word("Ravi had been saying that he had been there"),
            Some("had".to_string())
        );
    }
}

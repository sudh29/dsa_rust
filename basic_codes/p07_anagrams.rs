pub fn is_anagram(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut ca: Vec<char> = a.to_lowercase().chars().collect();
    let mut cb: Vec<char> = b.to_lowercase().chars().collect();
    ca.sort_unstable();
    cb.sort_unstable();
    ca == cb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagrams() {
        assert!(is_anagram("dance", "cadne"));
        assert!(!is_anagram("sudh", "Rama"));
    }
}

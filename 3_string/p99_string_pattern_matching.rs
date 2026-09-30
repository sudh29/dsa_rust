/// Performs naive string pattern matching, returning the starting index of `pattern` in `text` if found.
///
/// # Complexity
/// - Time Complexity: $O(N \times M)$
/// - Space Complexity: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::string::p99_string_pattern_matching::pattern_match_naive;
///
/// assert_eq!(pattern_match_naive("aaaaaabc", "abc"), Some(5));
/// assert_eq!(pattern_match_naive("aaaaaabc", "xyz"), None);
/// ```
pub fn pattern_match_naive(text: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    if p.len() > t.len() {
        return None;
    }
    for i in 0..=(t.len() - p.len()) {
        if &t[i..i + p.len()] == p {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_match() {
        assert_eq!(pattern_match_naive("aaaaaabc", "abc"), Some(5));
        assert_eq!(pattern_match_naive("aaaaaabc", "xyz"), None);
        assert_eq!(pattern_match_naive("hello", ""), Some(0));
        assert_eq!(pattern_match_naive("", "a"), None);
    }
}

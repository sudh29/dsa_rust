pub fn pattern_match_naive(text: &str, pattern: &str) -> isize {
    if pattern.is_empty() {
        return 0;
    }
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    if p.len() > t.len() {
        return -1;
    }
    for i in 0..=(t.len() - p.len()) {
        if &t[i..i + p.len()] == p {
            return i as isize;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_match() {
        assert_eq!(pattern_match_naive("aaaaaabc", "abc"), 5);
        assert_eq!(pattern_match_naive("aaaaaabc", "xyz"), -1);
    }
}

pub fn remove_consecutive_characters(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if !res.ends_with(c) {
            res.push(c);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_consecutive() {
        assert_eq!(remove_consecutive_characters("aabb"), "ab");
        assert_eq!(remove_consecutive_characters("aabaa"), "aba");
    }
}

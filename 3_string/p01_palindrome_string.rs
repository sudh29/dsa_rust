pub fn is_palindrome(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    for i in 0..n / 2 {
        if b[i] != b[n - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("abba"));
        assert!(!is_palindrome("abc"));
    }
}

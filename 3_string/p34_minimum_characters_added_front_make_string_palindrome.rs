pub fn min_char_add_to_front_palindrome(s: &str) -> usize {
    let rev: String = s.chars().rev().collect();
    let concat = format!("{}${}", s, rev);
    let b = concat.as_bytes();
    let n = b.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    let mut i = 1;

    while i < n {
        if b[i] == b[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    s.len() - lps[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_add() {
        assert_eq!(min_char_add_to_front_palindrome("ABC"), 2);
        assert_eq!(min_char_add_to_front_palindrome("AACECAAAA"), 2);
    }
}

pub fn longest_palindrome(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return String::new();
    }
    let mut start = 0;
    let mut max_len = 1;
    let mut dp = vec![vec![false; n]; n];
    for i in 0..n {
        dp[i][i] = true;
    }
    for i in 0..n - 1 {
        if b[i] == b[i + 1] {
            dp[i][i + 1] = true;
            if max_len < 2 {
                start = i;
                max_len = 2;
            }
        }
    }
    for len in 3..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if b[i] == b[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                if len > max_len {
                    start = i;
                    max_len = len;
                }
            }
        }
    }
    s[start..start + max_len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palin_substr() {
        assert_eq!(longest_palindrome("babad"), "bab");
        assert_eq!(longest_palindrome("cbbd"), "bb");
    }
}

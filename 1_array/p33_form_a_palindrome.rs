pub fn find_min_insertions(s: &str) -> usize {
    let n = s.len();
    if n <= 1 {
        return 0;
    }
    let bytes = s.as_bytes();
    let mut dp = vec![vec![0; n]; n];

    for gap in 1..n {
        let mut l = 0;
        for h in gap..n {
            if bytes[l] == bytes[h] {
                dp[l][h] = dp[l + 1][h - 1];
            } else {
                dp[l][h] = dp[l][h - 1].min(dp[l + 1][h]) + 1;
            }
            l += 1;
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_insertions() {
        assert_eq!(find_min_insertions("abcd"), 3);
        assert_eq!(find_min_insertions("aba"), 0);
        assert_eq!(find_min_insertions("geeks"), 3);
    }
}

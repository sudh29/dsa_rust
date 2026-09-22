pub fn is_interleave(a: &str, b: &str, c: &str) -> bool {
    let (ba, bb, bc) = (a.as_bytes(), b.as_bytes(), c.as_bytes());
    let (n, m, l) = (ba.len(), bb.len(), bc.len());
    if n + m != l {
        return false;
    }
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[0][0] = true;
    for j in 1..=m {
        dp[0][j] = dp[0][j - 1] && bb[j - 1] == bc[j - 1];
    }
    for i in 1..=n {
        dp[i][0] = dp[i - 1][0] && ba[i - 1] == bc[i - 1];
    }
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = (dp[i - 1][j] && ba[i - 1] == bc[i + j - 1])
                || (dp[i][j - 1] && bb[j - 1] == bc[i + j - 1]);
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interleave() {
        assert!(is_interleave("aabcc", "dbbca", "aadbbcbcac"));
        assert!(!is_interleave("aabcc", "dbbca", "aadbbbaccc"));
    }
}

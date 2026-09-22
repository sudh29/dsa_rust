pub fn lcs_of_3(a: &str, b: &str, c: &str) -> usize {
    let (ba, bb, bc) = (a.as_bytes(), b.as_bytes(), c.as_bytes());
    let (n1, n2, n3) = (ba.len(), bb.len(), bc.len());
    let mut dp = vec![vec![vec![0; n3 + 1]; n2 + 1]; n1 + 1];

    for i in 1..=n1 {
        for j in 1..=n2 {
            for k in 1..=n3 {
                if ba[i - 1] == bb[j - 1] && bb[j - 1] == bc[k - 1] {
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                } else {
                    dp[i][j][k] = dp[i - 1][j][k].max(dp[i][j - 1][k]).max(dp[i][j][k - 1]);
                }
            }
        }
    }
    dp[n1][n2][n3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_of_3() {
        assert_eq!(lcs_of_3("geeks", "geeksfor", "geeksforgeeks"), 5);
        assert_eq!(lcs_of_3("abcd1e2", "bc12ea", "bd1ea"), 3);
    }
}

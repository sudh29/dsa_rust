pub fn maximum_path(n: usize, mat: &[Vec<i32>]) -> i32 {
    let mut dp = mat.to_vec();
    for r in 1..n {
        for c in 0..n {
            let mut best = dp[r - 1][c];
            if c > 0 {
                best = best.max(dp[r - 1][c - 1]);
            }
            if c + 1 < n {
                best = best.max(dp[r - 1][c + 1]);
            }
            dp[r][c] += best;
        }
    }
    dp[n - 1].iter().copied().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_path() {
        let mat = vec![vec![348, 491], vec![618, 422]];
        assert_eq!(maximum_path(2, &mat), 1109);
    }
}

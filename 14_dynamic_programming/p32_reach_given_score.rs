pub fn reach_score(n: usize) -> i64 {
    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;
    for &move_pts in &[3, 5, 10] {
        for i in move_pts..=n {
            dp[i] += dp[i - move_pts];
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(reach_score(20), 4);
        assert_eq!(reach_score(13), 2);
    }
}

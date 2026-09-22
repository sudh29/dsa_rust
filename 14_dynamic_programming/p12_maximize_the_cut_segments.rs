pub fn maximize_the_cuts(n: usize, x: usize, y: usize, z: usize) -> i32 {
    let mut dp = vec![-1; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        for &seg in &[x, y, z] {
            if i >= seg && dp[i - seg] != -1 {
                dp[i] = dp[i].max(dp[i - seg] + 1);
            }
        }
    }
    dp[n].max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuts() {
        assert_eq!(maximize_the_cuts(4, 2, 1, 1), 4);
        assert_eq!(maximize_the_cuts(5, 5, 3, 2), 2);
    }
}

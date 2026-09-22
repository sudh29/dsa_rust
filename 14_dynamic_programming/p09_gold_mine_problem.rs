pub fn max_gold(n: usize, m: usize, mat: &[Vec<i32>]) -> i32 {
    let mut dp = mat.to_vec();
    for col in (0..m.saturating_sub(1)).rev() {
        for row in 0..n {
            let right = dp[row][col + 1];
            let right_up = if row > 0 { dp[row - 1][col + 1] } else { 0 };
            let right_down = if row + 1 < n { dp[row + 1][col + 1] } else { 0 };
            dp[row][col] += right.max(right_up).max(right_down);
        }
    }
    (0..n).map(|r| dp[r][0]).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gold_mine() {
        let mat = vec![vec![1, 3, 3], vec![2, 1, 4], vec![0, 6, 4]];
        assert_eq!(max_gold(3, 3, &mat), 12);
    }
}

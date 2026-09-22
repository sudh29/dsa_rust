pub fn max_square(n: usize, m: usize, mat: &[Vec<i32>]) -> usize {
    let mut dp = vec![vec![0; m]; n];
    let mut max_side = 0;
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 1 {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
                max_side = max_side.max(dp[i][j]);
            }
        }
    }
    max_side
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_square() {
        let mat = vec![
            vec![0, 1, 1, 0, 1],
            vec![1, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(max_square(6, 5, &mat), 3);
    }
}

pub fn egg_drop(n: usize, k: usize) -> usize {
    if n == 1 {
        return k;
    }
    if k <= 1 {
        return k;
    }
    let mut dp = vec![vec![0; k + 1]; n + 1];
    for row in dp.iter_mut().take(n + 1).skip(1) {
        row[1] = 1;
    }
    for (j, val) in dp[1].iter_mut().enumerate().take(k + 1).skip(1) {
        *val = j;
    }
    for i in 2..=n {
        for j in 2..=k {
            dp[i][j] = usize::MAX;
            for x in 1..=j {
                let res = 1 + dp[i - 1][x - 1].max(dp[i][j - x]);
                if res < dp[i][j] {
                    dp[i][j] = res;
                }
            }
        }
    }
    dp[n][k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_egg_drop() {
        assert_eq!(egg_drop(2, 10), 4);
        assert_eq!(egg_drop(1, 2), 2);
    }
}

pub fn matrix_multiplication(n: usize, arr: &[i32]) -> i32 {
    let mut dp = vec![vec![0; n]; n];
    for len in 2..n {
        for i in 1..=(n - len) {
            let j = i + len - 1;
            dp[i][j] = i32::MAX;
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + arr[i - 1] * arr[k] * arr[j];
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                }
            }
        }
    }
    dp[1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let arr = vec![10, 30, 5, 60];
        assert_eq!(matrix_multiplication(arr.len(), &arr), 4500);
        let arr2 = vec![40, 20, 30, 10, 30];
        assert_eq!(matrix_multiplication(arr2.len(), &arr2), 26000);
    }
}

pub fn minimum_cost(n: usize, w: usize, cost: &[i32]) -> i64 {
    const INF: i64 = 1_000_000_000_000;
    let mut dp = vec![INF; w + 1];
    dp[0] = 0;
    for i in 1..=n {
        if cost[i - 1] != -1 {
            let c = cost[i - 1] as i64;
            for j in i..=w {
                if dp[j - i] != INF {
                    dp[j] = dp[j].min(dp[j - i] + c);
                }
            }
        }
    }
    if dp[w] == INF {
        -1
    } else {
        dp[w]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost() {
        assert_eq!(minimum_cost(5, 5, &[20, 10, 4, 50, 100]), 14);
        assert_eq!(minimum_cost(5, 5, &[-1, -1, 4, 5, -1]), -1);
    }
}

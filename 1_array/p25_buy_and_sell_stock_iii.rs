pub fn max_profit_iii(prices: &[i32]) -> i32 {
    let n = prices.len();
    if n <= 1 {
        return 0;
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    let mut min_left = prices[0];
    for i in 1..n {
        min_left = min_left.min(prices[i]);
        left[i] = left[i - 1].max(prices[i] - min_left);
    }

    let mut max_right = prices[n - 1];
    for i in (0..n - 1).rev() {
        max_right = max_right.max(prices[i]);
        right[i] = right[i + 1].max(max_right - prices[i]);
    }

    let mut ans = right[0];
    for i in 0..n - 1 {
        ans = ans.max(left[i] + right[i + 1]);
    }
    ans.max(left[n - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_iii() {
        assert_eq!(max_profit_iii(&[3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(max_profit_iii(&[1, 2, 3, 4, 5]), 4);
    }
}

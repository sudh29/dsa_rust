pub fn max_profit_two_tx(price: &[i32]) -> i32 {
    let n = price.len();
    if n == 0 {
        return 0;
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    let mut min_p = price[0];
    for i in 1..n {
        min_p = min_p.min(price[i]);
        left[i] = left[i - 1].max(price[i] - min_p);
    }

    let mut max_p = price[n - 1];
    for i in (0..n - 1).rev() {
        max_p = max_p.max(price[i]);
        right[i] = right[i + 1].max(max_p - price[i]);
    }

    (0..n).map(|i| left[i] + right[i]).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_tx() {
        assert_eq!(max_profit_two_tx(&[10, 22, 5, 75, 65, 80]), 87);
        assert_eq!(max_profit_two_tx(&[2, 30, 15, 10, 8, 25, 80]), 100);
    }
}

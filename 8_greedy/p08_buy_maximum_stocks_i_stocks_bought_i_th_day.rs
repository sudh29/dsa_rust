pub fn buy_maximum_products(prices: &[i32], mut k: i32) -> i32 {
    let mut stocks: Vec<(i32, usize)> = prices
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i + 1))
        .collect();
    stocks.sort_by_key(|x| x.0);

    let mut total_bought = 0;
    for (price, max_can_buy) in stocks {
        let can_buy = (k / price).min(max_can_buy as i32);
        total_bought += can_buy;
        k -= can_buy * price;
    }
    total_bought
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_stocks() {
        assert_eq!(buy_maximum_products(&[10, 7, 19], 45), 4);
        assert_eq!(buy_maximum_products(&[7, 10, 4], 100), 6);
    }
}

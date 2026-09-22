pub fn count_subarrays_product_less_than_k(a: &[i64], k: i64) -> usize {
    if k <= 1 {
        return 0;
    }
    let mut prod = 1i64;
    let mut start = 0;
    let mut count = 0;
    for end in 0..a.len() {
        prod *= a[end];
        while start <= end && prod >= k {
            prod /= a[start];
            start += 1;
        }
        count += end - start + 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_less_than_k() {
        assert_eq!(count_subarrays_product_less_than_k(&[1, 2, 3, 4], 10), 7);
        assert_eq!(
            count_subarrays_product_less_than_k(&[1, 9, 2, 8, 6, 4, 3], 100),
            16
        );
    }
}

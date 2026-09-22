pub fn find_min_sum_abs_diff(a: &mut [i64], b: &mut [i64]) -> i64 {
    a.sort_unstable();
    b.sort_unstable();
    a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_abs_diff_pairs() {
        let mut a = [4, 1, 8, 7];
        let mut b = [2, 3, 6, 5];
        assert_eq!(find_min_sum_abs_diff(&mut a, &mut b), 6);
    }
}

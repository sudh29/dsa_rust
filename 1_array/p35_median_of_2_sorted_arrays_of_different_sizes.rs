pub fn median_of_arrays(a: &[i32], b: &[i32]) -> f64 {
    let mut combined: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    combined.sort_unstable();
    let n = combined.len();
    if n.is_multiple_of(2) {
        (combined[n / 2] + combined[n / 2 - 1]) as f64 / 2.0
    } else {
        combined[n / 2] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_two_arrays() {
        assert_eq!(median_of_arrays(&[1, 5, 9], &[2, 3, 6, 7]), 5.0);
        assert_eq!(median_of_arrays(&[4, 6], &[1, 2, 3, 5]), 3.5);
    }
}

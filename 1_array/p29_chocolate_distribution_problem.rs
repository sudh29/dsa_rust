pub fn find_min_diff(a: &mut [i32], m: usize) -> i32 {
    let n = a.len();
    if m == 0 || n == 0 || m > n {
        return 0;
    }
    a.sort_unstable();
    let mut min_diff = i32::MAX;
    for i in 0..=n - m {
        min_diff = min_diff.min(a[i + m - 1] - a[i]);
    }
    min_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chocolate_diff() {
        let mut a = [3, 4, 1, 9, 56, 7, 9, 12];
        assert_eq!(find_min_diff(&mut a, 5), 6);
    }
}

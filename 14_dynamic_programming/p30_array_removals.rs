pub fn removals(mut arr: Vec<i32>, k: i32) -> usize {
    arr.sort_unstable();
    let n = arr.len();
    let mut min_removals = n;
    let mut j = 0;
    for i in 0..n {
        while j < n && arr[j] - arr[i] <= k {
            j += 1;
        }
        min_removals = min_removals.min(n - (j - i));
    }
    min_removals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removals() {
        assert_eq!(removals(vec![1, 3, 4, 9, 10, 11, 12, 17, 20], 4), 5);
        assert_eq!(removals(vec![1, 5, 6, 2, 8], 2), 3);
    }
}

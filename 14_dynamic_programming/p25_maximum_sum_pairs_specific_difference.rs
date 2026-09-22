pub fn max_sum_pair_with_diff_less_than_k(mut arr: Vec<i32>, k: i32) -> i32 {
    arr.sort_unstable();
    let mut max_sum = 0;
    let mut i = arr.len() as isize - 1;
    while i > 0 {
        let idx = i as usize;
        if arr[idx] - arr[idx - 1] < k {
            max_sum += arr[idx] + arr[idx - 1];
            i -= 2;
        } else {
            i -= 1;
        }
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_diff() {
        assert_eq!(
            max_sum_pair_with_diff_less_than_k(vec![3, 5, 10, 15, 17, 12, 9], 4),
            62
        );
        assert_eq!(
            max_sum_pair_with_diff_less_than_k(vec![5, 15, 10, 300], 12),
            25
        );
    }
}

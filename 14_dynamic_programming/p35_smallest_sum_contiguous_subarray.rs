pub fn smallest_sum_subarray(arr: &[i32]) -> i32 {
    let mut min_so_far = arr[0];
    let mut curr = arr[0];
    for &x in arr.iter().skip(1) {
        curr = x.min(curr + x);
        min_so_far = min_so_far.min(curr);
    }
    min_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_sum() {
        assert_eq!(smallest_sum_subarray(&[3, -4, 2, -3, -1, 7, -5]), -6);
        assert_eq!(smallest_sum_subarray(&[2, 6, 8, 1, 4]), 1);
    }
}

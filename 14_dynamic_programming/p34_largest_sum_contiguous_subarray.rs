pub fn max_sub_array_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut curr = arr[0];
    for &x in arr.iter().skip(1) {
        curr = x.max(curr + x);
        max_so_far = max_so_far.max(curr);
    }
    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kadane() {
        assert_eq!(max_sub_array_sum(&[-2, -3, 4, -1, -2, 1, 5, -3]), 7);
        assert_eq!(max_sub_array_sum(&[-1, -2, -3, -4]), -1);
    }
}

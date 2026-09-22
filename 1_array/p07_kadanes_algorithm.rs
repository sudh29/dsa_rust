pub fn max_sub_array_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut curr_max = arr[0];
    for &x in &arr[1..] {
        curr_max = x.max(curr_max + x);
        max_so_far = max_so_far.max(curr_max);
    }
    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kadane() {
        assert_eq!(max_sub_array_sum(&[1, 2, 3, -2, 5]), 9);
        assert_eq!(max_sub_array_sum(&[-1, -2, -3, -4]), -1);
    }
}

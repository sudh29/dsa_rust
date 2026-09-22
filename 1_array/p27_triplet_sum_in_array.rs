pub fn find3_numbers(arr: &mut [i32], target: i32) -> bool {
    arr.sort_unstable();
    let n = arr.len();
    for i in 0..n.saturating_sub(2) {
        let mut left = i + 1;
        let mut right = n - 1;
        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == target {
                return true;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triplet_sum() {
        let mut arr = [1, 4, 45, 6, 10, 8];
        assert!(find3_numbers(&mut arr, 13));
        let mut arr2 = [1, 2, 4, 3, 6];
        assert!(!find3_numbers(&mut arr2, 100));
    }
}

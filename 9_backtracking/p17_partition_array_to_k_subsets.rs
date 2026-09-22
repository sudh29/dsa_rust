pub fn is_k_partition_possible(arr: &[i32], k: usize) -> bool {
    let sum: i32 = arr.iter().sum();
    if k == 0 || sum % k as i32 != 0 {
        return false;
    }
    let target = sum / k as i32;
    let mut subset_sums = vec![0; k];
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| b.cmp(a));

    fn can_partition(idx: usize, arr: &[i32], target: i32, subset_sums: &mut [i32]) -> bool {
        if idx == arr.len() {
            return true;
        }
        for i in 0..subset_sums.len() {
            if subset_sums[i] + arr[idx] <= target {
                subset_sums[i] += arr[idx];
                if can_partition(idx + 1, arr, target, subset_sums) {
                    return true;
                }
                subset_sums[i] -= arr[idx];
            }
            if subset_sums[i] == 0 {
                break;
            }
        }
        false
    }

    can_partition(0, &sorted_arr, target, &mut subset_sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_subsets() {
        assert!(is_k_partition_possible(&[2, 1, 4, 5, 6], 3));
        assert!(!is_k_partition_possible(&[2, 1, 5, 5, 6], 3));
    }
}

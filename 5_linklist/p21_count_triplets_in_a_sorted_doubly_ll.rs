pub fn count_triplets_dll(sorted_vals: &[i32], x: i32) -> usize {
    let n = sorted_vals.len();
    let mut count = 0;

    for i in 0..n {
        let mut left = i + 1;
        let mut right = n.saturating_sub(1);
        while left < right {
            let sum = sorted_vals[i] + sorted_vals[left] + sorted_vals[right];
            if sum == x {
                count += 1;
                left += 1;
                right -= 1;
            } else if sum < x {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triplets_dll() {
        assert_eq!(count_triplets_dll(&[1, 2, 4, 5, 6, 8, 9], 17), 2);
    }
}

pub fn find_pairs_dll(sorted_vals: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();
    if sorted_vals.is_empty() {
        return pairs;
    }
    let mut left = 0;
    let mut right = sorted_vals.len() - 1;

    while left < right {
        let sum = sorted_vals[left] + sorted_vals[right];
        if sum == target {
            pairs.push((sorted_vals[left], sorted_vals[right]));
            left += 1;
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_dll() {
        assert_eq!(
            find_pairs_dll(&[1, 2, 4, 5, 6, 8, 9], 7),
            vec![(1, 6), (2, 5)]
        );
    }
}

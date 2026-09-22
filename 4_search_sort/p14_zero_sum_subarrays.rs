use std::collections::HashMap;

pub fn find_zero_sum_subarrays(arr: &[i64]) -> i64 {
    let mut map = HashMap::new();
    map.insert(0, 1i64);
    let mut sum = 0;
    let mut count = 0;

    for &x in arr {
        sum += x;
        if let Some(&c) = map.get(&sum) {
            count += c;
        }
        *map.entry(sum).or_insert(0) += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_sum() {
        assert_eq!(find_zero_sum_subarrays(&[0, 0, 5, 5, 0, 0]), 6);
        assert_eq!(
            find_zero_sum_subarrays(&[6, -1, -3, 4, -2, 2, 4, 6, -12, -7]),
            4
        );
    }
}

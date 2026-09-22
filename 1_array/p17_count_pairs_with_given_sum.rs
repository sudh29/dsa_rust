use std::collections::HashMap;

pub fn get_pairs_count(arr: &[i32], k: i32) -> i32 {
    let mut freq = HashMap::new();
    let mut count = 0;
    for &x in arr {
        let complement = k - x;
        if let Some(&c) = freq.get(&complement) {
            count += c;
        }
        *freq.entry(x).or_insert(0) += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_count() {
        assert_eq!(get_pairs_count(&[1, 5, 7, 1], 6), 2);
        assert_eq!(get_pairs_count(&[1, 1, 1, 1], 2), 6);
    }
}

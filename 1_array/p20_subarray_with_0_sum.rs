use std::collections::HashSet;

pub fn sub_array_exists(arr: &[i32]) -> bool {
    let mut sum_val = 0;
    let mut seen = HashSet::new();
    seen.insert(0);
    for &x in arr {
        sum_val += x;
        if seen.contains(&sum_val) {
            return true;
        }
        seen.insert(sum_val);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_array_exists() {
        assert!(sub_array_exists(&[4, 2, -3, 1, 6]));
        assert!(!sub_array_exists(&[1, 2, 3, 4, 5]));
    }
}

pub fn get_min_max(arr: &[i32]) -> (i32, i32) {
    let mut min_val = i32::MAX;
    let mut max_val = i32::MIN;
    for &val in arr {
        if val < min_val {
            min_val = val;
        }
        if val > max_val {
            max_val = val;
        }
    }
    (min_val, max_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_min_max() {
        assert_eq!(get_min_max(&[3, 2, 1, 56, 10000, 167]), (1, 10000));
    }
}

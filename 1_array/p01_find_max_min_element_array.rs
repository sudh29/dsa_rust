pub fn get_min_max(a: &[i32]) -> (i32, i32) {
    let mut min_val = i32::MAX;
    let mut max_val = i32::MIN;
    for &val in a {
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
    fn test_min_max() {
        assert_eq!(get_min_max(&[3, 2, 1, 56, 10000, 167]), (1, 10000));
        assert_eq!(get_min_max(&[1, 345, 234, 21, 56789]), (1, 56789));
        assert_eq!(get_min_max(&[5]), (5, 5));
    }
}

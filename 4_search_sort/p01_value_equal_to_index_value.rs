pub fn value_equal_to_index(arr: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    for (i, &val) in arr.iter().enumerate() {
        if val == (i + 1) as i32 {
            res.push(val);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_equal_index() {
        assert_eq!(value_equal_to_index(&[15, 2, 45, 12, 7]), vec![2]);
        assert_eq!(value_equal_to_index(&[1]), vec![1]);
    }
}

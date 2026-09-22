pub fn split_circular(vals: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let n = vals.len();
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let mid = n.div_ceil(2);
    (vals[..mid].to_vec(), vals[mid..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_circular() {
        assert_eq!(split_circular(&[1, 5, 7]), (vec![1, 5], vec![7]));
        assert_eq!(split_circular(&[2, 6, 1, 5]), (vec![2, 6], vec![1, 5]));
    }
}

pub fn rotate_dll(mut vals: Vec<i32>, p: usize) -> Vec<i32> {
    if vals.is_empty() || p == 0 {
        return vals;
    }
    let p = p % vals.len();
    vals.rotate_left(p);
    vals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_dll() {
        assert_eq!(rotate_dll(vec![1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
    }
}

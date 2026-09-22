pub fn delete_and_reverse(mut vals: Vec<i32>, key: i32) -> Vec<i32> {
    if let Some(pos) = vals.iter().position(|&x| x == key) {
        vals.remove(pos);
    }
    vals.reverse();
    vals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_reverse() {
        assert_eq!(
            delete_and_reverse(vec![2, 5, 7, 8, 10], 8),
            vec![10, 7, 5, 2]
        );
    }
}

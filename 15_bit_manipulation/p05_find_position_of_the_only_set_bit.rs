pub fn find_position(n: u32) -> i32 {
    if n == 0 || (n & (n - 1)) != 0 {
        return -1;
    }
    n.trailing_zeros() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        assert_eq!(find_position(16), 5);
        assert_eq!(find_position(2), 2);
        assert_eq!(find_position(5), -1);
    }
}

pub fn count_squares(n: i32) -> i32 {
    let mut a = (n as f64).sqrt() as i32;
    if a * a == n {
        a -= 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_squares() {
        assert_eq!(count_squares(9), 2);
        assert_eq!(count_squares(3), 1);
        assert_eq!(count_squares(5), 2);
    }
}

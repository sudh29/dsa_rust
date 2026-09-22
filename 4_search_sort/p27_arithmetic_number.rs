pub fn in_sequence(a: i32, b: i32, c: i32) -> bool {
    if c == 0 {
        return a == b;
    }
    (b - a) % c == 0 && (b - a) / c >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        assert!(in_sequence(1, 3, 2));
        assert!(!in_sequence(1, 2, 3));
    }
}

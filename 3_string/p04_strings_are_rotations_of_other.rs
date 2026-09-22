pub fn are_rotations(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let doubled = format!("{}{}", s1, s1);
    doubled.contains(s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotations() {
        assert!(are_rotations("ABCD", "CDAB"));
        assert!(!are_rotations("ABCD", "ACBD"));
    }
}

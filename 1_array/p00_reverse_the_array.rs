pub fn reverse_word(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn reverse_array<T: Copy>(arr: &mut [T]) {
    arr.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_word() {
        assert_eq!(reverse_word("Geeks"), "skeeG");
        assert_eq!(reverse_word("for"), "rof");
    }

    #[test]
    fn test_reverse_array() {
        let mut a = [1, 2, 3, 4, 5];
        reverse_array(&mut a);
        assert_eq!(a, [5, 4, 3, 2, 1]);
    }
}

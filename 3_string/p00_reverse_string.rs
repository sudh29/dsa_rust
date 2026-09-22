pub fn reverse_string(s: &mut [char]) {
    s.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let mut s = ['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, ['o', 'l', 'l', 'e', 'h']);
    }
}

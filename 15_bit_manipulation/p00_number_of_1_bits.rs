pub fn set_bits(n: u32) -> u32 {
    n.count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bits() {
        assert_eq!(set_bits(6), 2);
        assert_eq!(set_bits(8), 1);
        assert_eq!(set_bits(15), 4);
    }
}

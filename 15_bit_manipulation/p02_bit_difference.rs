pub fn count_bits_flip(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_diff() {
        assert_eq!(count_bits_flip(10, 20), 4);
        assert_eq!(count_bits_flip(20, 25), 3);
    }
}

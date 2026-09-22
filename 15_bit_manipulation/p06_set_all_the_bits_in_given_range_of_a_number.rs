pub fn set_all_range_bits(n: u32, l: u32, r: u32) -> u32 {
    let mask = ((1 << (r - l + 1)) - 1) << (l - 1);
    n | mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_range() {
        assert_eq!(set_all_range_bits(17, 2, 3), 23);
        assert_eq!(set_all_range_bits(8, 1, 2), 11);
    }
}

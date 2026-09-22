pub fn count_total_set_bits(n: u32) -> u32 {
    let mut count = 0;
    let mut i = 0;
    while (1u64 << i) <= n as u64 {
        let block_size = 1u64 << (i + 1);
        let total_pairs = (n as u64 + 1) / block_size;
        let remainder = (n as u64 + 1) % block_size;
        let half_block = 1u64 << i;
        count += total_pairs * half_block + remainder.saturating_sub(half_block);
        i += 1;
    }
    count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_set_bits() {
        assert_eq!(count_total_set_bits(4), 5);
        assert_eq!(count_total_set_bits(17), 35);
        assert_eq!(count_total_set_bits(1), 1);
        assert_eq!(count_total_set_bits(0), 0);
    }
}

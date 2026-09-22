pub fn sum_of_digits(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

pub fn pointer_width_bits() -> usize {
    std::mem::size_of::<usize>() * 8
}

pub fn is_little_endian() -> bool {
    cfg!(target_endian = "little")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_details() {
        assert_eq!(sum_of_digits(12345), 15);
        assert!(pointer_width_bits() == 64 || pointer_width_bits() == 32);
        assert!(is_little_endian());
    }
}

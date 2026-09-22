pub fn swap_xor(mut a: i32, mut b: i32) -> (i32, i32) {
    std::mem::swap(&mut a, &mut b);
    (a, b)
}

pub fn swap_arithmetic(mut a: i32, mut b: i32) -> (i32, i32) {
    a += b;
    b = a - b;
    a -= b;
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swaps() {
        assert_eq!(swap_xor(100, 50), (50, 100));
        assert_eq!(swap_arithmetic(100, 50), (50, 100));
    }
}

pub fn square(mut n: i32) -> i32 {
    if n < 0 {
        n = -n;
    }
    let mut res = 0;
    let mut x = n;
    let mut pos = 0;

    while x > 0 {
        if (x & 1) != 0 {
            res += n << pos;
        }
        pos += 1;
        x >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(5), 25);
        assert_eq!(square(-7), 49);
        assert_eq!(square(0), 0);
    }
}

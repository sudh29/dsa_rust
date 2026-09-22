pub fn divide(dividend: i64, divisor: i64) -> i64 {
    if divisor == 0 {
        return i64::MAX;
    }
    let sign = if (dividend < 0) ^ (divisor < 0) {
        -1
    } else {
        1
    };
    let mut a = dividend.abs();
    let b = divisor.abs();
    let mut quotient = 0;

    for i in (0..=31).rev() {
        if (b << i) <= a {
            a -= b << i;
            quotient |= 1i64 << i;
        }
    }
    sign * quotient
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 3), 3);
        assert_eq!(divide(43, -8), -5);
    }
}

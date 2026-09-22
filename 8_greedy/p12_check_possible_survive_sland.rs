pub fn minimum_days_to_survive(s: i32, n: i32, m: i32) -> i32 {
    if m > n || (s > 6 && (n * 6) < (m * 7)) {
        return -1;
    }
    let total = s * m;
    let mut res = total / n;
    if total % n > 0 {
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_survive() {
        assert_eq!(minimum_days_to_survive(10, 16, 2), 2);
        assert_eq!(minimum_days_to_survive(10, 9, 8), -1);
    }
}

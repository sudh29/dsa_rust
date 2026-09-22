pub fn find_trailing_zeros(n: i32) -> i32 {
    let check = |p: i32| -> i32 {
        let mut count = 0;
        let mut f = 5;
        while f <= p {
            count += p / f;
            f *= 5;
        }
        count
    };

    if n == 1 {
        return 5;
    }
    let mut low = 0;
    let mut high = 5 * n;
    let mut ans = low;

    while low <= high {
        let mid = (low + high) / 2;
        if check(mid) >= n {
            ans = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(find_trailing_zeros(1), 5);
        assert_eq!(find_trailing_zeros(6), 25);
    }
}

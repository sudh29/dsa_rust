pub fn max_substr_equal_01(s: &str) -> i32 {
    let mut count0 = 0;
    let mut count1 = 0;
    let mut ans = 0;

    for c in s.chars() {
        if c == '0' {
            count0 += 1;
        } else if c == '1' {
            count1 += 1;
        }
        if count0 == count1 {
            ans += 1;
        }
    }
    if count0 != count1 {
        -1
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_binary() {
        assert_eq!(max_substr_equal_01("0100110101"), 4);
        assert_eq!(max_substr_equal_01("0111100010"), 3);
        assert_eq!(max_substr_equal_01("0000"), -1);
    }
}

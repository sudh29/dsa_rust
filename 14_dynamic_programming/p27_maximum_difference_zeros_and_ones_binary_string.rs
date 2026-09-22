pub fn max_substring_zeros_ones(s: &str) -> i32 {
    let mut max_sum = i32::MIN;
    let mut curr_sum = 0;
    for ch in s.chars() {
        let val = if ch == '0' { 1 } else { -1 };
        curr_sum += val;
        max_sum = max_sum.max(curr_sum);
        if curr_sum < 0 {
            curr_sum = 0;
        }
    }
    if max_sum > 0 {
        max_sum
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_substr() {
        assert_eq!(max_substring_zeros_ones("11000010001"), 6);
        assert_eq!(max_substring_zeros_ones("1111"), -1);
    }
}

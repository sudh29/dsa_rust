pub fn count_rev(s: &str) -> i32 {
    if !s.len().is_multiple_of(2) {
        return -1;
    }
    let mut left = 0;
    let mut right = 0;

    for c in s.chars() {
        if c == '{' {
            left += 1;
        } else if left > 0 {
            left -= 1;
        } else {
            right += 1;
        }
    }
    ((left + 1) / 2) + ((right + 1) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_rev() {
        assert_eq!(count_rev("}{{}}{{{"), 3);
        assert_eq!(count_rev("{{}{{{}{{}}{{"), -1);
    }
}

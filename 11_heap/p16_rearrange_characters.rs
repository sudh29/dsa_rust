pub fn rearrange_chars(s: &str) -> bool {
    let res =
        crate::string::p33_rearrange_characters_string_no_two_adjacent_are_same::rearrange_string(
            s,
        );
    !res.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rearrange() {
        assert!(rearrange_chars("aab"));
        assert!(!rearrange_chars("aaab"));
    }
}

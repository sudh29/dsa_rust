pub fn reorganize_string(s: &str) -> String {
    crate::string::p33_rearrange_characters_string_no_two_adjacent_are_same::rearrange_string(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorganize() {
        assert_eq!(reorganize_string("aab"), "aba");
        assert_eq!(reorganize_string("aaab"), "");
    }
}

pub fn rearrange_string(s: &str) -> String {
    crate::string::p33_rearrange_characters_string_no_two_adjacent_are_same::rearrange_string(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rearrange() {
        assert_eq!(rearrange_string("aab"), "aba");
    }
}

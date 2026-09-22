pub fn first_non_repeating(a: &str) -> String {
    crate::linklist::p35_first_non_repeating_character_in_a_stream::first_non_repeating_stream(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream() {
        assert_eq!(first_non_repeating("aabc"), "a#bb");
    }
}

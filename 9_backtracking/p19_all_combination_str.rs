pub fn string_combinations(s: &str) -> Vec<String> {
    crate::string::p09_print_all_subsequences_string::get_subsequences(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        assert_eq!(string_combinations("ab").len(), 4);
    }
}

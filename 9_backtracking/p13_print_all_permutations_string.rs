pub fn find_permutation(s: &str) -> Vec<String> {
    crate::string::p10_all_permutations_string::find_permutations(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        assert_eq!(find_permutation("ABC").len(), 6);
    }
}

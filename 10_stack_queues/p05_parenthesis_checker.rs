pub fn is_par(x: &str) -> bool {
    crate::string::p15_balanced_parenthesis_problem_imp::is_par(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par() {
        assert!(is_par("{([])}"));
        assert!(!is_par("([]"));
    }
}

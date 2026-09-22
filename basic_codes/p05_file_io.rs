pub fn simulated_file_io(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulated_io() {
        let text = "line1
line2
line3";
        assert_eq!(simulated_file_io(text), vec!["line1", "line2", "line3"]);
    }
}

pub fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',').map(|s| s.trim().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let line = "rock, paper, scissors";
        assert_eq!(parse_csv_line(line), vec!["rock", "paper", "scissors"]);
    }
}

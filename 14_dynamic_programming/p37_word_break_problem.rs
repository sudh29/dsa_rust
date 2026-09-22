use std::collections::HashSet;

pub fn word_break(s: &str, word_dict: &[String]) -> bool {
    let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        let dict = vec![
            "i".into(),
            "like".into(),
            "sam".into(),
            "sung".into(),
            "samsung".into(),
        ];
        assert!(word_break("ilikesamsung", &dict));
        assert!(!word_break("ilikeand", &dict));
    }
}

use std::collections::HashSet;

pub fn word_break(s: &str, word_dict: &[String]) -> bool {
    let set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && set.contains(&s[j..i]) {
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
        let dict = vec!["apple".to_string(), "pen".to_string()];
        assert!(word_break("applepenapple", &dict));
        let dict2 = vec!["cats".to_string(), "dog".to_string()];
        assert!(!word_break("catsdogcat", &dict2));
    }
}

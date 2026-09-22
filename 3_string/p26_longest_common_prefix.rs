pub fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first = &strs[0];
    for (i, (byte_idx, c)) in first.char_indices().enumerate() {
        for s in &strs[1..] {
            if let Some(other_c) = s.chars().nth(i) {
                if other_c != c {
                    return first[..byte_idx].to_string();
                }
            } else {
                return first[..byte_idx].to_string();
            }
        }
    }
    first.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcp() {
        let v = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(&v), "fl");
    }
}

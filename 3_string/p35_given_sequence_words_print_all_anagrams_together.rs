use std::collections::HashMap;

pub fn group_anagrams(words: &[String]) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for w in words {
        let mut chars: Vec<char> = w.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        map.entry(key).or_default().push(w.clone());
    }
    let mut res: Vec<Vec<String>> = map.into_values().collect();
    for group in res.iter_mut() {
        group.sort();
    }
    res.sort_by(|a, b| a[0].cmp(&b[0]));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagrams() {
        let words = vec![
            "act".to_string(),
            "god".to_string(),
            "cat".to_string(),
            "dog".to_string(),
            "tac".to_string(),
        ];
        let grouped = group_anagrams(&words);
        assert_eq!(grouped.len(), 2);
    }
}

use std::collections::HashMap;

pub fn anagrams(string_list: &[String]) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for word in string_list {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        map.entry(key).or_default().push(word.clone());
    }
    let mut result: Vec<Vec<String>> = map.into_values().collect();
    result.sort_by(|a, b| a[0].cmp(&b[0]));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagrams() {
        let words = vec![
            "act".into(),
            "god".into(),
            "cat".into(),
            "dog".into(),
            "tac".into(),
        ];
        let grouped = anagrams(&words);
        assert_eq!(grouped.len(), 2);
    }
}

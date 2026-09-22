pub fn get_subsequences(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    fn backtrack(chars: &[char], idx: usize, curr: &mut String, res: &mut Vec<String>) {
        if idx == chars.len() {
            res.push(curr.clone());
            return;
        }
        backtrack(chars, idx + 1, curr, res);
        curr.push(chars[idx]);
        backtrack(chars, idx + 1, curr, res);
        curr.pop();
    }
    let mut curr = String::new();
    backtrack(&chars, 0, &mut curr, &mut res);
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsequences() {
        let subs = get_subsequences("abc");
        assert_eq!(subs.len(), 8);
        assert!(subs.contains(&"abc".to_string()));
        assert!(subs.contains(&"".to_string()));
    }
}

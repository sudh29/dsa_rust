pub fn find_permutations(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    let mut res = Vec::new();

    fn backtrack(chars: &mut [char], idx: usize, res: &mut Vec<String>) {
        if idx == chars.len() {
            res.push(chars.iter().collect());
            return;
        }
        for i in idx..chars.len() {
            chars.swap(idx, i);
            backtrack(chars, idx + 1, res);
            chars.swap(idx, i);
        }
    }
    backtrack(&mut chars, 0, &mut res);
    res.sort();
    res.dedup();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let perms = find_permutations("ABC");
        assert_eq!(perms.len(), 6);
        assert!(perms.contains(&"ABC".to_string()));
        assert!(perms.contains(&"CBA".to_string()));
    }
}

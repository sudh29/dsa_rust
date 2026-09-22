pub fn all_palindromic_partitions(s: &str) -> Vec<Vec<String>> {
    fn is_palin(s: &str) -> bool {
        let b = s.as_bytes();
        let n = b.len();
        for i in 0..n / 2 {
            if b[i] != b[n - 1 - i] {
                return false;
            }
        }
        true
    }

    fn backtrack(s: &str, start: usize, curr: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
        if start == s.len() {
            res.push(curr.clone());
            return;
        }
        for end in start + 1..=s.len() {
            let sub = &s[start..end];
            if is_palin(sub) {
                curr.push(sub.to_string());
                backtrack(s, end, curr, res);
                curr.pop();
            }
        }
    }

    let mut res = Vec::new();
    let mut curr = Vec::new();
    backtrack(s, 0, &mut curr, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palin_partitions() {
        let sol = all_palindromic_partitions("geeks");
        assert_eq!(sol.len(), 2);
        assert!(sol.contains(&vec![
            "g".to_string(),
            "e".to_string(),
            "e".to_string(),
            "k".to_string(),
            "s".to_string()
        ]));
        assert!(sol.contains(&vec![
            "g".to_string(),
            "ee".to_string(),
            "k".to_string(),
            "s".to_string()
        ]));
    }
}

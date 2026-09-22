pub fn sentences_from_words(lists: &[Vec<&str>]) -> Vec<String> {
    let mut res = Vec::new();
    fn dfs(lists: &[Vec<&str>], idx: usize, current: &mut Vec<String>, res: &mut Vec<String>) {
        if idx == lists.len() {
            res.push(current.join(" "));
            return;
        }
        for &word in &lists[idx] {
            current.push(word.to_string());
            dfs(lists, idx + 1, current, res);
            current.pop();
        }
    }
    let mut curr = Vec::new();
    dfs(lists, 0, &mut curr, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentences() {
        let lists = vec![vec!["you", "we"], vec!["have", "are"], vec!["sleep", "eat"]];
        let sentences = sentences_from_words(&lists);
        assert_eq!(sentences.len(), 8);
        assert!(sentences.contains(&"you have sleep".to_string()));
    }
}

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

#[derive(Default)]
pub struct WordBreakTrie {
    root: TrieNode,
}

impl WordBreakTrie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            curr = curr.children.entry(ch).or_default();
        }
        curr.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut curr = &self.root;
        for ch in word.chars() {
            match curr.children.get(&ch) {
                Some(next) => curr = next,
                None => return false,
            }
        }
        curr.is_end
    }
}

pub fn word_break(s: &str, word_dict: &[String]) -> bool {
    let mut trie = WordBreakTrie::new();
    for w in word_dict {
        trie.insert(w);
    }
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && trie.search(&s[j..i]) {
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
    fn test_trie_word_break() {
        let dict = vec!["i".into(), "like".into(), "sam".into(), "sung".into()];
        assert!(word_break(
            "ilikesamsung",
            &[dict.clone(), vec!["samsung".into()]].concat()
        ));
        assert!(!word_break("ilikeand", &dict));
    }
}

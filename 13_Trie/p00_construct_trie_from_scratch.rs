use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end_of_word: bool,
}

#[derive(Default, Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: &str) {
        let mut curr = &mut self.root;
        for ch in key.chars() {
            curr = curr.children.entry(ch).or_default();
        }
        curr.is_end_of_word = true;
    }

    pub fn search(&self, key: &str) -> bool {
        let mut curr = &self.root;
        for ch in key.chars() {
            match curr.children.get(&ch) {
                Some(next) => curr = next,
                None => return false,
            }
        }
        curr.is_end_of_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("the");
        trie.insert("a");
        trie.insert("there");
        assert!(trie.search("the"));
        assert!(trie.search("there"));
        assert!(!trie.search("their"));
    }
}

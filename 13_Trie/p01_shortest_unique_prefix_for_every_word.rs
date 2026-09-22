use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    count: usize,
}

#[derive(Default)]
pub struct PrefixTrie {
    root: TrieNode,
}

impl PrefixTrie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            curr = curr.children.entry(ch).or_default();
            curr.count += 1;
        }
    }

    pub fn find_unique_prefix(&self, word: &str) -> String {
        let mut prefix = String::new();
        let mut curr = &self.root;
        for ch in word.chars() {
            if let Some(next) = curr.children.get(&ch) {
                prefix.push(ch);
                if next.count == 1 {
                    return prefix;
                }
                curr = next;
            } else {
                break;
            }
        }
        prefix
    }
}

pub fn shortest_unique_prefixes(words: &[String]) -> Vec<String> {
    let mut trie = PrefixTrie::new();
    for w in words {
        trie.insert(w);
    }
    words.iter().map(|w| trie.find_unique_prefix(w)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_prefix() {
        let words = vec!["zebra".into(), "dog".into(), "duck".into(), "dove".into()];
        assert_eq!(
            shortest_unique_prefixes(&words),
            vec![
                "z".to_string(),
                "dog".to_string(),
                "du".to_string(),
                "dov".to_string()
            ]
        );
    }
}

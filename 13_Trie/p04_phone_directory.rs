use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    contacts: BTreeSet<String>,
}

#[derive(Default)]
pub struct PhoneDirectory {
    root: TrieNode,
}

impl PhoneDirectory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            curr = curr.children.entry(ch).or_default();
            curr.contacts.insert(word.to_string());
        }
    }

    pub fn search(&self, prefix: &str) -> Vec<String> {
        let mut curr = &self.root;
        for ch in prefix.chars() {
            match curr.children.get(&ch) {
                Some(next) => curr = next,
                None => return vec!["0".to_string()],
            }
        }
        curr.contacts.iter().cloned().collect()
    }
}

pub fn display_contacts(contact: &[String], s: &str) -> Vec<Vec<String>> {
    let mut dir = PhoneDirectory::new();
    for c in contact {
        dir.insert(c);
    }
    let mut result = Vec::new();
    let mut prefix = String::new();
    for ch in s.chars() {
        prefix.push(ch);
        result.push(dir.search(&prefix));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_dir() {
        let contacts = vec![
            "geeikistest".into(),
            "geeksforgeeks".into(),
            "geeksfortest".into(),
        ];
        let res = display_contacts(&contacts, "gee");
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].len(), 3);
    }
}

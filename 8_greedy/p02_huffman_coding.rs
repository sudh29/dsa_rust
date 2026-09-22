use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct HuffNode {
    freq: usize,
    ch: Option<char>,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn huffman_codes(chars: &[char], freqs: &[usize]) -> Vec<String> {
    let mut heap = BinaryHeap::new();
    for (&c, &f) in chars.iter().zip(freqs.iter()) {
        heap.push(Reverse(HuffNode {
            freq: f,
            ch: Some(c),
            left: None,
            right: None,
        }));
    }

    while heap.len() > 1 {
        let Reverse(left) = heap.pop().unwrap();
        let Reverse(right) = heap.pop().unwrap();
        let parent = HuffNode {
            freq: left.freq + right.freq,
            ch: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        heap.push(Reverse(parent));
    }

    let root = heap.pop().unwrap().0;
    let mut codes = Vec::new();
    fn dfs(node: &HuffNode, code: &mut String, codes: &mut Vec<String>) {
        if node.left.is_none() && node.right.is_none() {
            codes.push(code.clone());
            return;
        }
        if let Some(ref l) = node.left {
            code.push('0');
            dfs(l, code, codes);
            code.pop();
        }
        if let Some(ref r) = node.right {
            code.push('1');
            dfs(r, code, codes);
            code.pop();
        }
    }
    let mut code = String::new();
    dfs(&root, &mut code, &mut codes);
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman() {
        let chars = ['a', 'b', 'c', 'd', 'e', 'f'];
        let freqs = [5, 9, 12, 13, 16, 45];
        let codes = huffman_codes(&chars, &freqs);
        assert_eq!(codes.len(), 6);
    }
}

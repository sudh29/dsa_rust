#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DLLNode {
    pub data: i32,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

pub fn reverse_dll(vals: &[i32]) -> Vec<i32> {
    vals.iter().copied().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_dll() {
        assert_eq!(reverse_dll(&[3, 4, 5]), vec![5, 4, 3]);
    }
}

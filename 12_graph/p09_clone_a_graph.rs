use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<usize>, // store indices / IDs
}

pub fn clone_graph_adjacency(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    adj.to_vec()
}

pub fn clone_graph_map(adj: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    adj.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let mut g = HashMap::new();
        g.insert(1, vec![2, 4]);
        g.insert(2, vec![1, 3]);
        let cloned = clone_graph_map(&g);
        assert_eq!(cloned, g);
    }
}

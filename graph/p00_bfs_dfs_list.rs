use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default, Debug)]
pub struct ListGraph {
    pub adj: HashMap<i32, Vec<i32>>,
}

impl ListGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, u: i32, v: i32) {
        self.adj.entry(u).or_default().push(v);
        self.adj.entry(v).or_default().push(u);
    }

    pub fn bfs(&self, start: i32) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut res = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            res.push(u);
            if let Some(neighbors) = self.adj.get(&u) {
                for &v in neighbors {
                    if visited.insert(v) {
                        queue.push_back(v);
                    }
                }
            }
        }
        res
    }

    pub fn dfs(&self, start: i32) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut res = Vec::new();

        fn dfs_helper(
            u: i32,
            adj: &HashMap<i32, Vec<i32>>,
            visited: &mut HashSet<i32>,
            res: &mut Vec<i32>,
        ) {
            visited.insert(u);
            res.push(u);
            if let Some(neighbors) = adj.get(&u) {
                for &v in neighbors {
                    if !visited.contains(&v) {
                        dfs_helper(v, adj, visited, res);
                    }
                }
            }
        }

        dfs_helper(start, &self.adj, &mut visited, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_graph() {
        let mut g = ListGraph::new();
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert_eq!(g.bfs(1), vec![1, 2, 3]);
        assert_eq!(g.dfs(1), vec![1, 2, 3]);
    }
}

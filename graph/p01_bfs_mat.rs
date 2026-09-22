use std::collections::VecDeque;

#[derive(Debug)]
pub struct MatrixGraph {
    pub size: usize,
    pub matrix: Vec<Vec<i32>>,
}

impl MatrixGraph {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            matrix: vec![vec![0; size]; size],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.matrix[u][v] = 1;
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.size];
        let mut queue = VecDeque::new();
        let mut path = Vec::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            path.push(u);
            for v in 0..self.size {
                if self.matrix[u][v] == 1 && !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat_graph() {
        let mut g = MatrixGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert_eq!(g.bfs(0), vec![0, 1, 2, 3]);
    }
}

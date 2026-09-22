use std::collections::VecDeque;

pub fn topo_sort(v: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    let mut in_degree = vec![0; v];
    for u in 0..v {
        for &neighbor in &adj[u] {
            in_degree[neighbor] += 1;
        }
    }
    let mut q = VecDeque::new();
    for i in 0..v {
        if in_degree[i] == 0 {
            q.push_back(i);
        }
    }
    let mut res = Vec::new();
    while let Some(u) = q.pop_front() {
        res.push(u);
        for &neighbor in &adj[u] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                q.push_back(neighbor);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort() {
        let adj = vec![vec![], vec![0], vec![0], vec![0]];
        let sorted = topo_sort(4, &adj);
        assert_eq!(sorted.len(), 4);
        assert_eq!(*sorted.last().unwrap(), 0);
    }
}

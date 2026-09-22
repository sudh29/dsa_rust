use std::collections::HashMap;

pub fn topological_sort_dfs(v_count: usize, graph: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; v_count];
    let mut stack = Vec::new();

    fn dfs(
        u: usize,
        graph: &HashMap<usize, Vec<usize>>,
        visited: &mut [bool],
        stack: &mut Vec<usize>,
    ) {
        visited[u] = true;
        if let Some(neighbors) = graph.get(&u) {
            for &n in neighbors {
                if !visited[n] {
                    dfs(n, graph, visited, stack);
                }
            }
        }
        stack.push(u);
    }

    for i in 0..v_count {
        if !visited[i] {
            dfs(i, graph, &mut visited, &mut stack);
        }
    }
    stack.reverse();
    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort_extra() {
        let mut g = HashMap::new();
        g.insert(5, vec![2, 0]);
        g.insert(4, vec![0, 1]);
        g.insert(2, vec![3]);
        g.insert(3, vec![1]);

        let sorted = topological_sort_dfs(6, &g);
        assert_eq!(sorted.len(), 6);
        let pos5 = sorted.iter().position(|&x| x == 5).unwrap();
        let pos0 = sorted.iter().position(|&x| x == 0).unwrap();
        assert!(pos5 < pos0);
    }
}

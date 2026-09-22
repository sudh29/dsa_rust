use std::collections::VecDeque;

pub fn minimum_time_jobs(n: usize, edges: &[[usize; 2]]) -> Vec<usize> {
    let mut adj = vec![Vec::new(); n + 1];
    let mut indegree = vec![0; n + 1];
    for &[u, v] in edges {
        adj[u].push(v);
        indegree[v] += 1;
    }
    let mut q = VecDeque::new();
    let mut ans = vec![0; n + 1];

    for i in 1..=n {
        if indegree[i] == 0 {
            q.push_back(i);
            ans[i] = 1;
        }
    }

    while let Some(u) = q.pop_front() {
        for &neighbor in &adj[u] {
            indegree[neighbor] -= 1;
            if indegree[neighbor] == 0 {
                ans[neighbor] = ans[u] + 1;
                q.push_back(neighbor);
            }
        }
    }
    ans[1..=n].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_time() {
        let edges = [
            [1, 3],
            [1, 4],
            [1, 5],
            [2, 3],
            [2, 8],
            [2, 9],
            [3, 6],
            [4, 6],
            [4, 8],
            [5, 8],
            [6, 7],
            [7, 8],
            [8, 10],
        ];
        let times = minimum_time_jobs(10, &edges);
        assert_eq!(times, vec![1, 1, 2, 2, 2, 3, 4, 5, 2, 6]);
    }
}

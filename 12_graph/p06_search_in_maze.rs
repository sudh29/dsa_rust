pub fn find_path_maze(m: &[Vec<i32>], n: usize) -> Vec<String> {
    let mut paths = Vec::new();
    if n == 0 || m[0][0] == 0 || m[n - 1][n - 1] == 0 {
        return paths;
    }
    let mut visited = vec![vec![false; n]; n];

    fn dfs(
        r: usize,
        c: usize,
        n: usize,
        m: &[Vec<i32>],
        visited: &mut [Vec<bool>],
        curr_path: &mut String,
        paths: &mut Vec<String>,
    ) {
        if r == n - 1 && c == n - 1 {
            paths.push(curr_path.clone());
            return;
        }
        visited[r][c] = true;
        let moves = [
            ('D', 1isize, 0isize),
            ('L', 0, -1),
            ('R', 0, 1),
            ('U', -1, 0),
        ];
        for (ch, dr, dc) in moves {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < n as isize && nc >= 0 && nc < n as isize {
                let (ur, uc) = (nr as usize, nc as usize);
                if m[ur][uc] == 1 && !visited[ur][uc] {
                    curr_path.push(ch);
                    dfs(ur, uc, n, m, visited, curr_path, paths);
                    curr_path.pop();
                }
            }
        }
        visited[r][c] = false;
    }

    let mut curr_path = String::new();
    dfs(0, 0, n, m, &mut visited, &mut curr_path, &mut paths);
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze() {
        let m = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 0, 1],
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 1],
        ];
        assert_eq!(
            find_path_maze(&m, 4),
            vec!["DDRDRR".to_string(), "DRDDRR".to_string()]
        );
    }
}

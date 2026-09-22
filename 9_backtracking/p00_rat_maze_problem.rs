pub fn find_path(m: &[Vec<i32>]) -> Vec<String> {
    let n = m.len();
    let mut res = Vec::new();
    if n == 0 || m[0][0] == 0 || m[n - 1][n - 1] == 0 {
        return res;
    }
    let mut visited = vec![vec![false; n]; n];

    fn dfs(
        r: usize,
        c: usize,
        n: usize,
        m: &[Vec<i32>],
        visited: &mut [Vec<bool>],
        path: &mut String,
        res: &mut Vec<String>,
    ) {
        if r == n - 1 && c == n - 1 {
            res.push(path.clone());
            return;
        }
        visited[r][c] = true;
        // D, L, R, U
        let dirs = [
            ('D', 1isize, 0isize),
            ('L', 0, -1),
            ('R', 0, 1),
            ('U', -1, 0),
        ];
        for (dir_ch, dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < n as isize && nc >= 0 && nc < n as isize {
                let (ur, uc) = (nr as usize, nc as usize);
                if m[ur][uc] == 1 && !visited[ur][uc] {
                    path.push(dir_ch);
                    dfs(ur, uc, n, m, visited, path, res);
                    path.pop();
                }
            }
        }
        visited[r][c] = false;
    }

    let mut path = String::new();
    dfs(0, 0, n, m, &mut visited, &mut path, &mut res);
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rat_maze() {
        let m = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 0, 1],
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 1],
        ];
        assert_eq!(find_path(&m), vec!["DDRDRR", "DRDDRR"]);
    }
}

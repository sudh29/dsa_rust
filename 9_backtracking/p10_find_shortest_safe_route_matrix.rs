use std::collections::VecDeque;

pub fn find_shortest_path(mat: &[Vec<i32>]) -> i32 {
    let r = mat.len();
    if r == 0 {
        return -1;
    }
    let c = mat[0].len();
    let mut safe = mat.to_vec();

    for i in 0..r {
        for j in 0..c {
            if mat[i][j] == 0 {
                let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dr, dc) in dirs {
                    let ni = i as isize + dr;
                    let nj = j as isize + dc;
                    if ni >= 0 && ni < r as isize && nj >= 0 && nj < c as isize {
                        safe[ni as usize][nj as usize] = 0;
                    }
                }
            }
        }
    }

    let mut dist = vec![vec![-1; c]; r];
    let mut q = VecDeque::new();
    for i in 0..r {
        if safe[i][0] == 1 {
            dist[i][0] = 1;
            q.push_back((i, 0));
        }
    }

    while let Some((cr, cc)) = q.pop_front() {
        if cc == c - 1 {
            return dist[cr][cc];
        }
        let dirs = [(-1, 0), (1, 0), (0, 1)];
        for (dr, dc) in dirs {
            let nr = cr as isize + dr;
            let nc = cc as isize + dc;
            if nr >= 0 && nr < r as isize && nc >= 0 && nc < c as isize {
                let (ur, uc) = (nr as usize, nc as usize);
                if safe[ur][uc] == 1 && dist[ur][uc] == -1 {
                    dist[ur][uc] = dist[cr][cc] + 1;
                    q.push_back((ur, uc));
                }
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_safe_route() {
        let mat = vec![vec![1, 1, 1, 1], vec![1, 0, 1, 1], vec![1, 1, 1, 1]];
        assert_eq!(find_shortest_path(&mat), -1);
    }
}

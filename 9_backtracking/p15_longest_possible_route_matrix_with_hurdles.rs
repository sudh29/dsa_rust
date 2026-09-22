pub fn longest_path(mat: &[Vec<i32>], xs: usize, ys: usize, xd: usize, yd: usize) -> i32 {
    let r = mat.len();
    if r == 0 || mat[xs][ys] == 0 || mat[xd][yd] == 0 {
        return -1;
    }
    let c = mat[0].len();
    let mut visited = vec![vec![false; c]; r];

    fn dfs(
        x: usize,
        y: usize,
        xd: usize,
        yd: usize,
        r: usize,
        c: usize,
        mat: &[Vec<i32>],
        visited: &mut [Vec<bool>],
        dist: i32,
        max_dist: &mut i32,
    ) {
        if x == xd && y == yd {
            *max_dist = (*max_dist).max(dist);
            return;
        }
        visited[x][y] = true;
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < r as isize && ny >= 0 && ny < c as isize {
                let (ux, uy) = (nx as usize, ny as usize);
                if mat[ux][uy] == 1 && !visited[ux][uy] {
                    dfs(ux, uy, xd, yd, r, c, mat, visited, dist + 1, max_dist);
                }
            }
        }
        visited[x][y] = false;
    }

    let mut max_dist = -1;
    dfs(xs, ys, xd, yd, r, c, mat, &mut visited, 0, &mut max_dist);
    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_path() {
        let mat = vec![vec![1, 1, 1, 1], vec![1, 1, 0, 1], vec![1, 1, 1, 1]];
        assert_eq!(longest_path(&mat, 0, 0, 1, 3), 8);
    }
}

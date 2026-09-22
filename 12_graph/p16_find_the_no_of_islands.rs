pub fn num_islands(grid: &[Vec<i32>]) -> usize {
    let r = grid.len();
    if r == 0 {
        return 0;
    }
    let c = grid[0].len();
    let mut visited = vec![vec![false; c]; r];
    let mut count = 0;

    fn dfs(x: usize, y: usize, r: usize, c: usize, grid: &[Vec<i32>], visited: &mut [Vec<bool>]) {
        visited[x][y] = true;
        let dirs = [
            (-1isize, -1isize),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < r as isize && ny >= 0 && ny < c as isize {
                let (ux, uy) = (nx as usize, ny as usize);
                if grid[ux][uy] == 1 && !visited[ux][uy] {
                    dfs(ux, uy, r, c, grid, visited);
                }
            }
        }
    }

    for i in 0..r {
        for j in 0..c {
            if grid[i][j] == 1 && !visited[i][j] {
                count += 1;
                dfs(i, j, r, c, grid, &mut visited);
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        let grid = vec![vec![0, 1], vec![1, 0], vec![1, 1], vec![1, 0]];
        assert_eq!(num_islands(&grid), 1);
    }
}

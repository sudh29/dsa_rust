pub fn unique_paths_obstacle_grid(a: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let r = a.len();
    if r == 0 {
        return Vec::new();
    }
    let c = a[0].len();
    let mut paths = vec![vec![0; c]; r];
    if a[0][0] == 0 {
        paths[0][0] = 1;
    }
    for i in 1..r {
        if a[i][0] == 0 {
            paths[i][0] = paths[i - 1][0];
        }
    }
    for j in 1..c {
        if a[0][j] == 0 {
            paths[0][j] = paths[0][j - 1];
        }
    }
    for i in 1..r {
        for j in 1..c {
            if a[i][j] == 0 {
                paths[i][j] = paths[i - 1][j] + paths[i][j - 1];
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_paths() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let res = unique_paths_obstacle_grid(&grid);
        assert_eq!(res[2][2], 2);
    }
}

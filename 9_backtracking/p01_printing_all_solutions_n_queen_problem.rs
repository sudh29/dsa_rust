pub fn n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    let mut board = vec![0; n];

    fn is_safe(board: &[usize], row: usize, col: usize) -> bool {
        for prev_col in 0..col {
            let prev_row = board[prev_col];
            if prev_row == row || prev_row.abs_diff(row) == col - prev_col {
                return false;
            }
        }
        true
    }

    fn solve(col: usize, n: usize, board: &mut [usize], res: &mut Vec<Vec<usize>>) {
        if col == n {
            let solution: Vec<usize> = board.iter().map(|&r| r + 1).collect();
            res.push(solution);
            return;
        }
        for row in 0..n {
            if is_safe(board, row, col) {
                board[col] = row;
                solve(col + 1, n, board, res);
            }
        }
    }

    solve(0, n, &mut board, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens() {
        let sol = n_queens(4);
        assert_eq!(sol.len(), 2);
        assert_eq!(sol[0], vec![2, 4, 1, 3]);
        assert_eq!(sol[1], vec![3, 1, 4, 2]);
    }
}

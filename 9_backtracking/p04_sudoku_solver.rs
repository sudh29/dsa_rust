pub fn solve_sudoku(grid: &mut [[u8; 9]; 9]) -> bool {
    fn is_valid(grid: &[[u8; 9]; 9], r: usize, c: usize, num: u8) -> bool {
        for i in 0..9 {
            if grid[r][i] == num || grid[i][c] == num {
                return false;
            }
        }
        let start_r = (r / 3) * 3;
        let start_c = (c / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if grid[start_r + i][start_c + j] == num {
                    return false;
                }
            }
        }
        true
    }

    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] == 0 {
                for num in 1..=9 {
                    if is_valid(grid, r, c, num) {
                        grid[r][c] = num;
                        if solve_sudoku(grid) {
                            return true;
                        }
                        grid[r][c] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku() {
        let mut board = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];
        assert!(solve_sudoku(&mut board));
        assert_eq!(board[0][1], 1);
    }
}

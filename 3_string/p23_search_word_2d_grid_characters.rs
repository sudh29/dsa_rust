pub fn search_word(grid: &[Vec<char>], word: &str) -> Vec<[usize; 2]> {
    let n = grid.len();
    if n == 0 {
        return Vec::new();
    }
    let m = grid[0].len();
    let w = word.as_bytes();
    let mut res = Vec::new();

    let dx = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dy = [-1, 0, 1, -1, 1, -1, 0, 1];

    for r in 0..n {
        for c in 0..m {
            if grid[r][c] as u8 != w[0] {
                continue;
            }
            for dir in 0..8 {
                let mut cr = r as isize;
                let mut cc = c as isize;
                let mut k = 0;
                while k < w.len() {
                    if cr < 0 || cr >= n as isize || cc < 0 || cc >= m as isize {
                        break;
                    }
                    if grid[cr as usize][cc as usize] as u8 != w[k] {
                        break;
                    }
                    cr += dx[dir];
                    cc += dy[dir];
                    k += 1;
                }
                if k == w.len() {
                    res.push([r, c]);
                    break;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_word() {
        let grid = vec![
            vec!['a', 'b', 'a', 'e', 'b', 'e', 'b', 'e'],
            vec!['e', 'b', 'a', 'e', 'b', 'a', 'e', 'b'],
            vec!['a', 'b', 'e', 'e', 'e', 'a', 'e', 'a'],
        ];
        let found = search_word(&grid, "abe");
        assert!(!found.is_empty());
    }
}

use std::collections::VecDeque;

pub fn min_step_to_reach_target(
    knight_pos: (usize, usize),
    target_pos: (usize, usize),
    n: usize,
) -> i32 {
    if knight_pos == target_pos {
        return 0;
    }
    let dirs = [
        (2isize, 1isize),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];
    let mut visited = vec![vec![false; n + 1]; n + 1];
    let mut queue = VecDeque::new();

    queue.push_back((knight_pos.0, knight_pos.1, 0));
    visited[knight_pos.0][knight_pos.1] = true;

    while let Some((x, y, dist)) = queue.pop_front() {
        if (x, y) == target_pos {
            return dist;
        }
        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 1 && nx <= n as isize && ny >= 1 && ny <= n as isize {
                let (ux, uy) = (nx as usize, ny as usize);
                if !visited[ux][uy] {
                    visited[ux][uy] = true;
                    queue.push_back((ux, uy, dist + 1));
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
    fn test_knight() {
        assert_eq!(min_step_to_reach_target((4, 5), (1, 1), 6), 3);
        assert_eq!(min_step_to_reach_target((1, 1), (1, 1), 6), 0);
    }
}

pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: usize, sc: usize, new_color: i32) -> Vec<Vec<i32>> {
    let old_color = image[sr][sc];
    if old_color == new_color {
        return image;
    }
    let r = image.len();
    let c = image[0].len();

    fn dfs(
        x: usize,
        y: usize,
        r: usize,
        c: usize,
        old_col: i32,
        new_col: i32,
        img: &mut [Vec<i32>],
    ) {
        if img[x][y] != old_col {
            return;
        }
        img[x][y] = new_col;
        let dirs = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < r as isize && ny >= 0 && ny < c as isize {
                dfs(nx as usize, ny as usize, r, c, old_col, new_col, img);
            }
        }
    }

    dfs(sr, sc, r, c, old_color, new_color, &mut image);
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let res = flood_fill(image, 1, 1, 2);
        assert_eq!(res, vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1],]);
    }
}

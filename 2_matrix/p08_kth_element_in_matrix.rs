pub fn kth_smallest(mat: &[Vec<i32>], k: usize) -> i32 {
    let mut flat = Vec::new();
    for row in mat {
        flat.extend(row);
    }
    flat.sort_unstable();
    flat[k - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        let mat = vec![
            vec![16, 28, 60, 64],
            vec![22, 41, 63, 91],
            vec![27, 50, 87, 93],
            vec![36, 78, 87, 94],
        ];
        assert_eq!(kth_smallest(&mat, 3), 27);
    }
}

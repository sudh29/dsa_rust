pub fn min_jumps(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    if arr[0] == 0 {
        return -1;
    }
    let mut jumps = 0;
    let mut current_end = 0;
    let mut farthest = 0;
    for i in 0..n - 1 {
        farthest = farthest.max(i + arr[i] as usize);
        if i == current_end {
            jumps += 1;
            current_end = farthest;
            if current_end >= n - 1 {
                return jumps;
            }
        }
    }
    if current_end >= n - 1 {
        jumps
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_jumps() {
        assert_eq!(min_jumps(&[1, 3, 5, 8, 9, 2, 6, 7, 6, 8, 9]), 3);
        assert_eq!(min_jumps(&[1, 4, 3, 2, 6, 7]), 2);
        assert_eq!(min_jumps(&[0, 10, 20]), -1);
    }
}

pub fn min_swap(arr: &[i32], k: i32) -> usize {
    let count = arr.iter().filter(|&&x| x <= k).count();
    if count <= 1 {
        return 0;
    }
    let mut bad = arr[0..count].iter().filter(|&&x| x > k).count();
    let mut ans = bad;

    for i in 0..(arr.len() - count) {
        if arr[i] > k {
            bad -= 1;
        }
        if arr[i + count] > k {
            bad += 1;
        }
        ans = ans.min(bad);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swap() {
        assert_eq!(min_swap(&[2, 1, 5, 6, 3], 3), 1);
        assert_eq!(min_swap(&[2, 7, 9, 5, 8, 7, 4], 6), 2);
    }
}

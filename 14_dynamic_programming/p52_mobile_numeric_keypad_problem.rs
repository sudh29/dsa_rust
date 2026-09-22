pub fn get_keypad_count(n: usize) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 10;
    }
    let moves = [
        vec![0, 8],          // 0
        vec![1, 2, 4],       // 1
        vec![2, 1, 3, 5],    // 2
        vec![3, 2, 6],       // 3
        vec![4, 1, 5, 7],    // 4
        vec![5, 2, 4, 6, 8], // 5
        vec![6, 3, 5, 9],    // 6
        vec![7, 4, 8],       // 7
        vec![8, 5, 7, 9, 0], // 8
        vec![9, 6, 8],       // 9
    ];
    let mut prev = [1i64; 10];
    let mut curr = vec![0i64; 10];

    for _ in 2..=n {
        for j in 0..10 {
            curr[j] = moves[j].iter().map(|&k| prev[k]).sum();
        }
        prev.copy_from_slice(&curr);
    }
    prev.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypad() {
        assert_eq!(get_keypad_count(1), 10);
        assert_eq!(get_keypad_count(2), 36);
    }
}

pub fn longest_increasing_subsequence(a: &[i32]) -> usize {
    let mut tails = Vec::new();
    for &num in a {
        match tails.binary_search(&num) {
            Ok(_) => {} // existing element
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(num);
                } else {
                    tails[pos] = num;
                }
            }
        }
    }
    tails.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lis() {
        assert_eq!(
            longest_increasing_subsequence(&[10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
        assert_eq!(longest_increasing_subsequence(&[0, 1, 0, 3, 2, 3]), 4);
    }
}

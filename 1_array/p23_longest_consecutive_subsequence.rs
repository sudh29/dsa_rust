use std::collections::HashSet;

pub fn find_longest_conseq_subseq(arr: &[i32]) -> usize {
    let set: HashSet<i32> = arr.iter().copied().collect();
    let mut max_len = 0;

    for &x in arr {
        if !set.contains(&(x - 1)) {
            let mut curr = x;
            let mut count = 1;
            while set.contains(&(curr + 1)) {
                curr += 1;
                count += 1;
            }
            max_len = max_len.max(count);
        }
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(find_longest_conseq_subseq(&[2, 6, 1, 9, 4, 5, 3]), 6);
        assert_eq!(find_longest_conseq_subseq(&[1, 9, 3, 10, 4, 20, 2]), 4);
    }
}

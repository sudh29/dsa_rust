pub fn min_swaps(nums: &[i32]) -> usize {
    let n = nums.len();
    let mut arr: Vec<(i32, usize)> = nums
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    arr.sort_unstable_by_key(|&(v, _)| v);

    let mut visited = vec![false; n];
    let mut swaps = 0;

    for i in 0..n {
        if visited[i] || arr[i].1 == i {
            continue;
        }
        let mut cycle_size = 0;
        let mut j = i;
        while !visited[j] {
            visited[j] = true;
            j = arr[j].1;
            cycle_size += 1;
        }
        if cycle_size > 1 {
            swaps += cycle_size - 1;
        }
    }
    swaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps() {
        assert_eq!(min_swaps(&[2, 8, 5, 4]), 1);
        assert_eq!(min_swaps(&[10, 19, 6, 3, 5]), 2);
    }
}

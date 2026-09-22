pub fn count_triplets(arr: &mut [i64], sum: i64) -> i64 {
    arr.sort_unstable();
    let n = arr.len();
    let mut ans = 0;

    for i in 0..n.saturating_sub(2) {
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            if arr[i] + arr[j] + arr[k] < sum {
                ans += (k - j) as i64;
                j += 1;
            } else {
                k -= 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_triplets() {
        let mut a = [-2, 0, 1, 3];
        assert_eq!(count_triplets(&mut a, 2), 2);
        let mut a2 = [5, 1, 3, 4, 7];
        assert_eq!(count_triplets(&mut a2, 12), 4);
    }
}

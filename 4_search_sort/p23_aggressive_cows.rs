pub fn aggressive_cows(stalls: &mut [i32], k: usize) -> i32 {
    stalls.sort_unstable();
    let n = stalls.len();
    let mut low = 1;
    let mut high = stalls[n - 1] - stalls[0];
    let mut ans = 0;

    let can_place = |dist: i32| -> bool {
        let mut count = 1;
        let mut last = stalls[0];
        for &s in &stalls[1..] {
            if s - last >= dist {
                count += 1;
                last = s;
                if count >= k {
                    return true;
                }
            }
        }
        false
    };

    while low <= high {
        let mid = low + (high - low) / 2;
        if can_place(mid) {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cows() {
        let mut stalls = [1, 2, 8, 4, 9];
        assert_eq!(aggressive_cows(&mut stalls, 3), 3);
    }
}

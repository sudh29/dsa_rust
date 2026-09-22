pub fn four_sum(arr: &mut [i32], k: i32) -> Vec<Vec<i32>> {
    arr.sort_unstable();
    let n = arr.len();
    let mut res = Vec::new();
    if n < 4 {
        return res;
    }

    for i in 0..n - 3 {
        if i > 0 && arr[i] == arr[i - 1] {
            continue;
        }
        for j in i + 1..n - 2 {
            if j > i + 1 && arr[j] == arr[j - 1] {
                continue;
            }
            let mut left = j + 1;
            let mut right = n - 1;
            while left < right {
                let sum = arr[i] as i64 + arr[j] as i64 + arr[left] as i64 + arr[right] as i64;
                if sum == k as i64 {
                    res.push(vec![arr[i], arr[j], arr[left], arr[right]]);
                    left += 1;
                    right -= 1;
                    while left < right && arr[left] == arr[left - 1] {
                        left += 1;
                    }
                    while left < right && arr[right] == arr[right + 1] {
                        right -= 1;
                    }
                } else if sum < k as i64 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum() {
        let mut a = [0, 0, 2, 1, 1];
        assert_eq!(four_sum(&mut a, 3), vec![vec![0, 0, 1, 2]]);
    }
}

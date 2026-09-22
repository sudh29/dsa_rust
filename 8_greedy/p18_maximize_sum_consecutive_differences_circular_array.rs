pub fn max_sum_consecutive_diff(arr: &mut [i64]) -> i64 {
    arr.sort_unstable();
    let n = arr.len();
    let mut ordered = Vec::new();
    let mut i = 0;
    let mut j = n - 1;

    while i <= j {
        if i == j {
            ordered.push(arr[i]);
            break;
        }
        ordered.push(arr[i]);
        ordered.push(arr[j]);
        i += 1;
        j -= 1;
    }

    let mut sum = 0;
    for k in 0..n {
        let next = (k + 1) % n;
        sum += (ordered[k] - ordered[next]).abs();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consec_diff() {
        let mut a = [4, 2, 1, 8];
        assert_eq!(max_sum_consecutive_diff(&mut a), 18);
    }
}

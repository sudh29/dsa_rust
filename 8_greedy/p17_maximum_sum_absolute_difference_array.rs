pub fn max_sum_abs_diff(n: usize) -> usize {
    let arr: Vec<usize> = (1..=n).collect();
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = n - 1;

    while i <= j {
        if i == j {
            res.push(arr[i]);
            break;
        }
        res.push(arr[i]);
        res.push(arr[j]);
        i += 1;
        j -= 1;
    }

    let mut sum = 0;
    for k in 0..n {
        let next = (k + 1) % n;
        sum += res[k].abs_diff(res[next]);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_abs_diff() {
        assert_eq!(max_sum_abs_diff(4), 8);
    }
}

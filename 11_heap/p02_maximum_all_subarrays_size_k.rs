use std::collections::VecDeque;

pub fn max_of_subarrays(arr: &[i32], k: usize) -> Vec<i32> {
    let mut dq = VecDeque::new();
    let mut res = Vec::new();

    for i in 0..arr.len() {
        if let Some(&front) = dq.front() {
            if front <= i.saturating_sub(k) {
                dq.pop_front();
            }
        }
        while let Some(&back) = dq.back() {
            if arr[back] <= arr[i] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        if i + 1 >= k {
            res.push(arr[*dq.front().unwrap()]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarrays() {
        let a = [1, 2, 3, 1, 4, 5, 2, 3, 6];
        assert_eq!(max_of_subarrays(&a, 3), vec![3, 3, 4, 5, 5, 5, 6]);
    }
}

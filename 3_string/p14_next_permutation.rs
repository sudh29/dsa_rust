pub fn next_permutation(arr: &mut [i32]) -> bool {
    let n = arr.len();
    if n <= 1 {
        return false;
    }
    let mut i = (n - 2) as isize;
    while i >= 0 && arr[i as usize] >= arr[(i + 1) as usize] {
        i -= 1;
    }
    if i < 0 {
        arr.reverse();
        return false;
    }
    let mut j = (n - 1) as isize;
    while arr[j as usize] <= arr[i as usize] {
        j -= 1;
    }
    arr.swap(i as usize, j as usize);
    arr[(i + 1) as usize..].reverse();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_perm() {
        let mut a = [1, 2, 3];
        next_permutation(&mut a);
        assert_eq!(a, [1, 3, 2]);
    }
}

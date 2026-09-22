pub fn rotate(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let last = arr[arr.len() - 1];
    for i in (1..arr.len()).rev() {
        arr[i] = arr[i - 1];
    }
    arr[0] = last;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut arr = [1, 2, 3, 4, 5];
        rotate(&mut arr);
        assert_eq!(arr, [5, 1, 2, 3, 4]);
    }
}

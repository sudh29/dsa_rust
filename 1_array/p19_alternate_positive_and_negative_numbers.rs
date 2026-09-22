pub fn rearrange(arr: &mut [i32]) {
    let pos: Vec<i32> = arr.iter().copied().filter(|&x| x >= 0).collect();
    let neg: Vec<i32> = arr.iter().copied().filter(|&x| x < 0).collect();

    let mut i = 0;
    let mut p = 0;
    let mut n = 0;

    while p < pos.len() && n < neg.len() {
        arr[i] = pos[p];
        i += 1;
        p += 1;
        arr[i] = neg[n];
        i += 1;
        n += 1;
    }
    while p < pos.len() {
        arr[i] = pos[p];
        i += 1;
        p += 1;
    }
    while n < neg.len() {
        arr[i] = neg[n];
        i += 1;
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rearrange() {
        let mut arr = [9, 4, -2, -1, 5, 0, -5, -3, 2];
        rearrange(&mut arr);
        assert_eq!(arr, [9, -2, 4, -1, 5, -5, 0, -3, 2]);
    }
}

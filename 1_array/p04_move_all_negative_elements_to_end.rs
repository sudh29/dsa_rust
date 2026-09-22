pub fn segregate_elements(arr: &mut [i32]) {
    let mut pos = Vec::new();
    let mut neg = Vec::new();
    for &x in arr.iter() {
        if x >= 0 {
            pos.push(x);
        } else {
            neg.push(x);
        }
    }
    let mut idx = 0;
    for x in pos {
        arr[idx] = x;
        idx += 1;
    }
    for x in neg {
        arr[idx] = x;
        idx += 1;
    }
}

pub fn move_negatives_two_pointer(arr: &mut [i32]) {
    let mut left = 0;
    let mut right = arr.len().saturating_sub(1);
    while left < right && right < arr.len() {
        if arr[left] < 0 {
            left += 1;
        } else if arr[right] >= 0 {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            arr.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segregate() {
        let mut arr = [1, -1, 3, 2, -7, -5, 11, 6];
        segregate_elements(&mut arr);
        assert_eq!(arr, [1, 3, 2, 11, 6, -1, -7, -5]);
    }
}

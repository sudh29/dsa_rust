pub fn sort012(arr: &mut [i32]) {
    let (mut low, mut mid) = (0, 0);
    let mut high = arr.len().saturating_sub(1);
    while mid <= high && high < arr.len() {
        match arr[mid] {
            0 => {
                arr.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => {
                mid += 1;
            }
            2 => {
                arr.swap(mid, high);
                if high == 0 {
                    break;
                }
                high -= 1;
            }
            _ => mid += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort012() {
        let mut arr = [0, 2, 1, 2, 0];
        sort012(&mut arr);
        assert_eq!(arr, [0, 0, 1, 2, 2]);

        let mut arr2 = [0, 1, 0];
        sort012(&mut arr2);
        assert_eq!(arr2, [0, 0, 1]);
    }
}

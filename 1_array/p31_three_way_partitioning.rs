pub fn three_way_partition(arr: &mut [i32], a: i32, b: i32) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = arr.len().saturating_sub(1);

    while mid <= high && high < arr.len() {
        if arr[mid] < a {
            arr.swap(low, mid);
            low += 1;
            mid += 1;
        } else if arr[mid] > b {
            arr.swap(mid, high);
            if high == 0 {
                break;
            }
            high -= 1;
        } else {
            mid += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_way_partition() {
        let mut arr = [1, 2, 3, 34, 98];
        three_way_partition(&mut arr, 1, 2);
        assert!(arr[0] <= 2);
        assert!(arr[1] <= 2);
    }
}

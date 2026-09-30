pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n.saturating_sub(i + 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut a = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut a);
        assert_eq!(a, [11, 12, 22, 25, 34, 64, 90]);
    }
}

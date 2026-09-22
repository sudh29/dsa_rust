pub fn max_heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n && arr[l] > arr[largest] {
        largest = l;
    }
    if r < n && arr[r] > arr[largest] {
        largest = r;
    }
    if largest != i {
        arr.swap(i, largest);
        max_heapify(arr, n, largest);
    }
}

pub fn build_max_heap(arr: &mut [i32]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        max_heapify(arr, n, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_heap() {
        let mut a = [1, 3, 5, 4, 6, 13, 10, 9, 8, 15, 17];
        build_max_heap(&mut a);
        assert_eq!(a[0], 17);
    }
}

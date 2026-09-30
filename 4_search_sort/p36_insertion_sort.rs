pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i as isize - 1;
        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut a = [12, 11, 13, 5, 6];
        insertion_sort(&mut a);
        assert_eq!(a, [5, 6, 11, 12, 13]);
    }
}

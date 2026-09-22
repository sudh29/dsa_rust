pub fn max_histogram_area(hist: &[i32]) -> i32 {
    let mut stack = Vec::new();
    let mut max_area = 0;
    let n = hist.len();

    for i in 0..=n {
        let h = if i == n { 0 } else { hist[i] };
        while let Some(&top) = stack.last() {
            if h < hist[top] {
                stack.pop();
                let height = hist[top];
                let width = if let Some(&prev) = stack.last() {
                    (i - prev - 1) as i32
                } else {
                    i as i32
                };
                max_area = max_area.max(height * width);
            } else {
                break;
            }
        }
        stack.push(i);
    }
    max_area
}

pub fn max_rectangle(mat: &[Vec<i32>]) -> i32 {
    if mat.is_empty() || mat[0].is_empty() {
        return 0;
    }
    let m = mat[0].len();
    let mut hist = vec![0; m];
    let mut max_area = 0;

    for row in mat {
        for j in 0..m {
            if row[j] == 0 {
                hist[j] = 0;
            } else {
                hist[j] += 1;
            }
        }
        max_area = max_area.max(max_histogram_area(&hist));
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_rectangle() {
        let mat = vec![
            vec![0, 1, 1, 0],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(max_rectangle(&mat), 8);
    }
}

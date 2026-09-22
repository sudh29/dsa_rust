pub fn max_stop(num_platforms: usize, trains: &[[usize; 3]]) -> usize {
    let mut plat_trains: Vec<Vec<(usize, usize)>> = vec![Vec::new(); num_platforms + 1];
    for &[arr, dep, plat] in trains {
        plat_trains[plat].push((arr, dep));
    }

    let mut count = 0;
    for list in plat_trains.iter_mut() {
        list.sort_by_key(|x| x.1); // sort by departure
        let mut last_dep = 0;
        for &(arr, dep) in list.iter() {
            if arr >= last_dep {
                count += 1;
                last_dep = dep;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trains() {
        let trains = [
            [1000, 1030, 1],
            [1010, 1020, 1],
            [1025, 1040, 1],
            [1000, 1020, 2],
            [1030, 1045, 2],
        ];
        assert_eq!(max_stop(2, &trains), 4);
    }
}

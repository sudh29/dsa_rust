use dsa_rust::common::dsu::DisjointSetUnion;
use dsa_rust::common::fenwick::FenwickTree;
use dsa_rust::common::lru_cache::LruCache;
use dsa_rust::common::segment_tree::SegmentTree;
use dsa_rust::search_sort::p02_search_in_a_rotated_sorted_array::search_rotated;
use dsa_rust::sorting_algorithms::{
    p00_bubble_sort::bubble_sort, p01_selection_sort::selection_sort,
    p02_insertion_sort::insertion_sort, p03_merge_sort::merge_sort, p04_quick_sort::quick_sort,
    p05_heap_sort::heap_sort,
};

#[test]
fn test_all_sorting_algorithms_invariants() {
    let test_cases: Vec<Vec<i32>> = vec![
        vec![],
        vec![42],
        vec![2, 1],
        vec![1, 2],
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
        vec![7, 7, 7, 7, 7],
        vec![-10, 50, -30, 0, 100, -5, 20],
        // Pseudo-random pseudo-permutation
        (0..100).map(|i| (i * 37 + 13) % 97).collect(),
    ];

    type Sorter = fn(&mut [i32]);
    let sorters: [(&str, Sorter); 6] = [
        ("bubble_sort", bubble_sort),
        ("selection_sort", selection_sort),
        ("insertion_sort", insertion_sort),
        ("merge_sort", merge_sort),
        ("quick_sort", quick_sort),
        ("heap_sort", heap_sort),
    ];

    for (name, sort_fn) in sorters {
        for original in &test_cases {
            let mut arr = original.clone();
            sort_fn(&mut arr);

            // Invariant 1: Length preservation
            assert_eq!(
                arr.len(),
                original.len(),
                "{}: length mismatch on {:?}",
                name,
                original
            );

            // Invariant 2: Monotonic non-decreasing order
            assert!(
                arr.windows(2).all(|w| w[0] <= w[1]),
                "{}: failed monotonic order on {:?}",
                name,
                arr
            );

            // Invariant 3: Multiset preservation
            let mut sorted_original = original.clone();
            sorted_original.sort();
            assert_eq!(
                arr, sorted_original,
                "{}: elements altered on {:?}",
                name, original
            );
        }
    }
}

#[test]
fn test_search_rotated_all_pivots_invariant() {
    let base = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    let n = base.len();

    // Test across every possible rotation offset
    for pivot in 0..n {
        let mut rotated = Vec::with_capacity(n);
        rotated.extend_from_slice(&base[pivot..]);
        rotated.extend_from_slice(&base[..pivot]);

        // Invariant 1: Every present element must be found at its exact index
        for (expected_idx, &val) in rotated.iter().enumerate() {
            let found = search_rotated(&rotated, val);
            assert_eq!(
                found,
                Some(expected_idx),
                "Failed to find {} at pivot {}",
                val,
                pivot
            );
        }

        // Invariant 2: Non-existent elements must return None
        for missing in [-5, 1, 3, 5, 7, 9, 25, 999] {
            assert_eq!(
                search_rotated(&rotated, missing),
                None,
                "False positive for {} at pivot {}",
                missing,
                pivot
            );
        }
    }
}

#[test]
fn test_dsu_mathematical_equivalence_invariants() {
    let n = 20;
    let mut dsu = DisjointSetUnion::new(n);

    // Reflexivity: x is connected to x
    for i in 0..n {
        assert!(dsu.connected(i, i));
    }

    // Connect in groups of 4: [0..4], [4..8], [8..12], etc.
    for group in 0..5 {
        let base = group * 4;
        dsu.union(base, base + 1);
        dsu.union(base + 1, base + 2);
        dsu.union(base + 2, base + 3);
    }

    assert_eq!(dsu.count(), 5);

    // Symmetry & Transitivity verification
    for i in 0..n {
        for j in 0..n {
            let same_group = (i / 4) == (j / 4);
            assert_eq!(
                dsu.connected(i, j),
                same_group,
                "Symmetry/transitivity check failed for ({}, {})",
                i,
                j
            );
            assert_eq!(dsu.connected(i, j), dsu.connected(j, i));
        }
    }
}

#[test]
fn test_segment_tree_differential_testing() {
    let arr = [12, -5, 8, 0, 19, -23, 44, 7, -1, 30];
    let n = arr.len();

    let mut st = SegmentTree::new(&arr, 0, |a, b| a + b);

    // Invariant: query(i, j) == sum(arr[i..=j]) for all pairs
    for i in 0..n {
        for j in i..n {
            let expected: i32 = arr[i..=j].iter().sum();
            assert_eq!(
                st.query(i, j),
                expected,
                "Range sum mismatch on [{}, {}]",
                i,
                j
            );
        }
    }

    // Update and re-verify
    st.update(3, 100);
    let mut modified_arr = arr;
    modified_arr[3] = 100;
    for i in 0..n {
        for j in i..n {
            let expected: i32 = modified_arr[i..=j].iter().sum();
            assert_eq!(
                st.query(i, j),
                expected,
                "Post-update mismatch on [{}, {}]",
                i,
                j
            );
        }
    }
}

#[test]
fn test_fenwick_tree_differential_testing() {
    let arr: Vec<i64> = vec![3, 2, -1, 6, 5, 4, -3, 3, 7, 2, 3];
    let n = arr.len();

    let mut ft = FenwickTree::from_slice(&arr);

    for i in 0..n {
        for j in i..n {
            let expected: i64 = arr[i..=j].iter().sum();
            assert_eq!(
                ft.range_sum(i, j),
                expected,
                "Fenwick sum mismatch on [{}, {}]",
                i,
                j
            );
        }
    }

    ft.add(4, 10);
    let mut modified_arr = arr;
    modified_arr[4] += 10;
    for i in 0..n {
        for j in i..n {
            let expected: i64 = modified_arr[i..=j].iter().sum();
            assert_eq!(
                ft.range_sum(i, j),
                expected,
                "Fenwick post-update mismatch on [{}, {}]",
                i,
                j
            );
        }
    }
}

#[test]
fn test_lru_cache_eviction_invariants() {
    let capacity = 3;
    let mut cache = LruCache::new(capacity);

    for i in 1..=10 {
        cache.put(i, i * 100);
        assert!(cache.len() <= capacity);
    }

    // Only last 3 elements (8, 9, 10) must remain
    for evicted in 1..=7 {
        assert_eq!(cache.get(&evicted), None);
    }
    assert_eq!(cache.get(&8), Some(&800));
    assert_eq!(cache.get(&9), Some(&900));
    assert_eq!(cache.get(&10), Some(&1000));
}

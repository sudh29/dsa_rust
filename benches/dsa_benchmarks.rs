use dsa_rust::array::p07_kadanes_algorithm::max_sub_array_sum;
use dsa_rust::common::dsu::DisjointSetUnion;
use dsa_rust::common::lru_cache::LruCache;
use dsa_rust::common::segment_tree::SegmentTree;
use dsa_rust::sorting_algorithms::{
    p02_insertion_sort::insertion_sort, p03_merge_sort::merge_sort, p04_quick_sort::quick_sort,
    p05_heap_sort::heap_sort,
};
use std::time::Instant;

fn main() {
    println!("============================================================");
    println!("          🦀 DSA Rust Empirical Benchmark Suite 🦀          ");
    println!("============================================================\n");

    bench_sorting();
    bench_segment_tree_vs_linear();
    bench_dsu();
    bench_lru_cache();
    bench_kadane();

    println!("============================================================");
    println!("                ✅ Benchmarks Completed                     ");
    println!("============================================================");
}

fn bench_sorting() {
    println!("--- 1. Sorting Algorithms Benchmark (N = 1,000 elements, 50 runs) ---");
    let base_data: Vec<i32> = (0..1000).map(|i| (i * 37 + 13) % 997).collect();

    // Insertion Sort
    let start = Instant::now();
    for _ in 0..50 {
        let mut d = base_data.clone();
        insertion_sort(&mut d);
    }
    let duration = start.elapsed();
    println!(
        "  InsertionSort: {:>10.2?} total ({:>8.2?} / run)",
        duration,
        duration / 50
    );

    // Merge Sort
    let start = Instant::now();
    for _ in 0..50 {
        let mut d = base_data.clone();
        merge_sort(&mut d);
    }
    let duration = start.elapsed();
    println!(
        "  MergeSort:     {:>10.2?} total ({:>8.2?} / run)",
        duration,
        duration / 50
    );

    // Heap Sort
    let start = Instant::now();
    for _ in 0..50 {
        let mut d = base_data.clone();
        heap_sort(&mut d);
    }
    let duration = start.elapsed();
    println!(
        "  HeapSort:      {:>10.2?} total ({:>8.2?} / run)",
        duration,
        duration / 50
    );

    // Quick Sort
    let start = Instant::now();
    for _ in 0..50 {
        let mut d = base_data.clone();
        quick_sort(&mut d);
    }
    let duration = start.elapsed();
    println!(
        "  QuickSort:     {:>10.2?} total ({:>8.2?} / run)",
        duration,
        duration / 50
    );

    // Standard Library Sort
    let start = Instant::now();
    for _ in 0..50 {
        let mut d = base_data.clone();
        d.sort();
    }
    let duration = start.elapsed();
    println!(
        "  std::sort:     {:>10.2?} total ({:>8.2?} / run)\n",
        duration,
        duration / 50
    );
}

fn bench_segment_tree_vs_linear() {
    println!(
        "--- 2. Range Sum Queries: SegmentTree vs Linear Scan (N = 1,000, 10,000 queries) ---"
    );
    let arr: Vec<i32> = (0..1000).map(|i| (i * 17) % 100).collect();
    let queries: Vec<(usize, usize)> = (0..10_000)
        .map(|i| {
            let l = (i * 13) % 950;
            let r = l + ((i * 7) % (999 - l));
            (l, r)
        })
        .collect();

    // Segment Tree
    let st = SegmentTree::new(&arr, 0, |a, b| a + b);
    let start = Instant::now();
    let mut sum_st: i64 = 0;
    for &(l, r) in &queries {
        sum_st += st.query(l, r) as i64;
    }
    let duration_st = start.elapsed();
    println!(
        "  SegmentTree O(log N): {:>10.2?} (checksum: {})",
        duration_st, sum_st
    );

    // Linear Scan
    let start = Instant::now();
    let mut sum_linear: i64 = 0;
    for &(l, r) in &queries {
        sum_linear += arr[l..=r].iter().map(|&x| x as i64).sum::<i64>();
    }
    let duration_linear = start.elapsed();
    println!(
        "  Linear Scan O(N):     {:>10.2?} (checksum: {})",
        duration_linear, sum_linear
    );
    let speedup = duration_linear.as_nanos() as f64 / duration_st.as_nanos().max(1) as f64;
    println!(
        "  Speedup:              {:.1}x faster with SegmentTree!\n",
        speedup
    );
}

fn bench_dsu() {
    println!("--- 3. Disjoint Set Union (DSU): 100,000 Operations ---");
    let mut dsu = DisjointSetUnion::new(20_000);
    let start = Instant::now();
    for i in 0..50_000 {
        let u = (i * 7) % 20_000;
        let v = (i * 13) % 20_000;
        dsu.union(u, v);
    }
    for i in 0..50_000 {
        let u = (i * 17) % 20_000;
        let v = (i * 23) % 20_000;
        let _ = dsu.connected(u, v);
    }
    let duration = start.elapsed();
    println!(
        "  Time for 100,000 union/find ops: {:>10.2?} (Components: {})\n",
        duration,
        dsu.count()
    );
}

fn bench_lru_cache() {
    println!("--- 4. LRU Cache: 100,000 Get & Put Operations (Capacity = 1,000) ---");
    let mut cache = LruCache::new(1000);
    let start = Instant::now();
    for i in 0..50_000 {
        cache.put(i % 2000, i * 2);
    }
    let mut hits = 0;
    for i in 0..50_000 {
        if cache.get(&(i % 2000)).is_some() {
            hits += 1;
        }
    }
    let duration = start.elapsed();
    println!(
        "  Time for 100,000 LRU ops: {:>10.2?} (Cache hits: {} / 50,000)\n",
        duration, hits
    );
}

fn bench_kadane() {
    println!("--- 5. Kadane's Maximum Subarray (N = 100,000 elements, 100 runs) ---");
    let arr: Vec<i32> = (0..100_000i32).map(|i| ((i * 31) % 201) - 100).collect();
    let start = Instant::now();
    let mut total_max = 0;
    for _ in 0..100 {
        total_max += max_sub_array_sum(&arr);
    }
    let duration = start.elapsed();
    println!(
        "  Kadane O(N): {:>10.2?} total ({:>8.2?} / run, max_sum: {})\n",
        duration,
        duration / 100,
        total_max / 100
    );
}

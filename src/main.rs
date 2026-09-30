use dsa_rust::array::p07_kadanes_algorithm::max_sub_array_sum;
use dsa_rust::common::dsu::DisjointSetUnion;
use dsa_rust::common::lru_cache::LruCache;
use dsa_rust::common::segment_tree::SegmentTree;
use dsa_rust::sorting_algorithms::p04_quick_sort::quick_sort;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "demo" => run_interactive_demo(),
        "categories" | "list" => list_categories(),
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("Unknown command: '{}'\n", other);
            print_help();
        }
    }
}

fn print_help() {
    println!("============================================================");
    println!("       🦀 DSA & Rust Learning Workspace CLI Runner 🦀       ");
    println!("============================================================");
    println!("A 100% Safe, production-grade Rust workspace featuring 370+");
    println!("Data Structures & Algorithms and 13 educational chapters in `chapters/`.\n");
    println!("USAGE:");
    println!("    cargo run -- <COMMAND>\n");
    println!("COMMANDS:");
    println!("    demo          Run live demonstrations of flagship algorithms");
    println!("    categories    List all 16 algorithm categories & problem counts");
    println!("    help          Display this help documentation\n");
    println!("COMMON WORKFLOWS:");
    println!("    cargo test --workspace      # Run all 410+ unit & integration tests");
    println!("    cargo test --doc            # Run all executable documentation tests");
    println!("    cargo clippy --workspace    # Check strict workspace lints (-D warnings)");
    println!("    cargo fmt --check           # Check formatting across all 440+ files");
}

fn list_categories() {
    println!("📚 Algorithm Categories in `dsa_rust`:\n");
    let categories = [
        (
            "1_array",
            35,
            "Two-pointer, Kadane, Sliding window, Dutch national flag",
        ),
        (
            "2_matrix",
            10,
            "Spiral traversal, matrix search, 2D Kadane, rotations",
        ),
        ("3_string", 35, "KMP, Rabin-Karp, Boyer-Moore, Palindromes"),
        (
            "4_search_sort",
            32,
            "Binary search, pivot search, 6 generic sorting algorithms",
        ),
        (
            "5_linklist",
            29,
            "Singly/doubly lists, Floyd cycle, reversals",
        ),
        (
            "6_binary_tree",
            41,
            "Traversals, views, LCA, diameter, level sums",
        ),
        (
            "7_bst",
            24,
            "BST search, insertion, deletion, AVL balancing",
        ),
        (
            "8_greedy",
            27,
            "Activity selection, Huffman, job scheduling",
        ),
        (
            "9_backtracking",
            22,
            "N-Queens, Sudoku, Rat in Maze, Tower of Hanoi",
        ),
        (
            "10_stack_queues",
            15,
            "Monotonic stack, min stack O(1), k-stacks",
        ),
        (
            "11_heap",
            20,
            "Min/Max heap structs, running median, k-way merge",
        ),
        (
            "12_graph",
            24,
            "Kahn's topo sort, Dijkstra, Kruskal MST, Floyd-Warshall",
        ),
        ("13_Trie", 6, "Trie prefix search, shortest unique prefix"),
        (
            "14_dynamic_programming",
            50,
            "Knapsack, LCS, LIS, MCM, Catalan, Egg drop",
        ),
        (
            "15_bit_manipulation",
            10,
            "Bit tricks, power set, non-repeating numbers",
        ),
        (
            "basic_codes",
            14,
            "Math basics, file I/O, anagrams, prime tests",
        ),
        (
            "common",
            7,
            "ListNode<T>, TreeNode<T>, Graph, DSU, SegmentTree, Fenwick, LruCache",
        ),
    ];

    println!("{:<24} {:<10} Highlights", "Category", "Problems");
    println!("{:-<24} {:-<10} {:-<45}", "", "", "");
    for (cat, count, desc) in categories {
        println!("{:<24} {:<10} {}", cat, count, desc);
    }
}

fn run_interactive_demo() {
    println!("🚀 Running Flagship DSA Demonstrations in Safe Rust...\n");

    // 1. Kadane's Algorithm
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_sub_array_sum(&arr);
    println!("1. Kadane's Maximum Subarray:");
    println!("   Input:  {:?}", arr);
    println!("   Output: {} (Subarray: [4, -1, 2, 1])\n", max_sum);

    // 2. Generic QuickSort
    let mut words = ["rust", "golang", "zig", "c", "python", "mojo"];
    println!("2. Generic QuickSort (<T: Ord>):");
    println!("   Input:  {:?}", words);
    quick_sort(&mut words);
    println!("   Output: {:?}\n", words);

    // 3. Segment Tree Range Query
    let numbers = [1, 3, 5, 7, 9, 11];
    let mut st = SegmentTree::new(&numbers, 0, |a, b| a + b);
    println!("3. Segment Tree Range Sum Query (O(log N)):");
    println!("   Array: {:?}", numbers);
    println!("   Range Sum [1..=3] (3+5+7): {}", st.query(1, 3));
    st.update(1, 10);
    println!(
        "   After updating index 1 to 10: Range Sum [1..=3]: {}\n",
        st.query(1, 3)
    );

    // 4. Disjoint Set Union (DSU)
    let mut dsu = DisjointSetUnion::new(6);
    dsu.union(0, 1);
    dsu.union(1, 2);
    dsu.union(3, 4);
    println!("4. Disjoint Set Union (DSU / Union-Find with Path Compression):");
    println!("   Unions: (0, 1), (1, 2), (3, 4)");
    println!("   Connected(0, 2): {}", dsu.connected(0, 2));
    println!("   Connected(0, 3): {}", dsu.connected(0, 3));
    println!("   Component Count: {}\n", dsu.count());

    // 5. LRU Cache
    let mut cache = LruCache::new(2);
    cache.put("key1", 100);
    cache.put("key2", 200);
    let _ = cache.get(&"key1"); // Access key1
    cache.put("key3", 300); // Evicts key2
    println!("5. LRU Cache (Capacity = 2):");
    println!("   Put(key1), Put(key2), Get(key1), Put(key3)");
    println!("   Get(key1): {:?}", cache.get(&"key1"));
    println!("   Get(key2): {:?} (evicted)", cache.get(&"key2"));
    println!("   Get(key3): {:?}", cache.get(&"key3"));

    println!("\n✅ All demonstrations executed successfully with zero runtime errors!");
}

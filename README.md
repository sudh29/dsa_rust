# DSA & Rust Learning Workspace

[![CI](https://github.com/sudh29/dsa_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/sudh29/dsa_rust/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Safety](https://img.shields.io/badge/unsafe-0%25-brightgreen.svg)](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html)
[![Clippy](https://img.shields.io/badge/clippy-zero--warnings-brightgreen.svg)](https://github.com/rust-lang/rust-clippy)
[![Tests](https://img.shields.io/badge/tests-435%20passed-brightgreen.svg)](tests/)

A comprehensive, production-grade repository featuring **395+ Data Structures & Algorithms (DSA)** solutions implemented in 100% safe, idiomatic Rust, alongside a structured 13-chapter **Learn Rust** tutorial suite, advanced generic primitives, differential invariant testing, and empirical benchmarks.

---

## 🚀 Quick Start

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Edition 2021, Rust 1.70+ recommended)

### Build, Test & Lint

```bash
# Build the entire workspace
cargo check --workspace --all-targets

# Run all 413 unit and integration invariant tests
cargo test --workspace

# Run all 22 executable documentation tests
cargo test --doc

# Run the interactive CLI runner & algorithm demonstrations
cargo run -- demo
cargo run -- categories

# Run the empirical benchmark suite
cargo bench

# Format code and enforce strict linter cleanliness (zero warnings)
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings

# Generate and view documentation
cargo doc --no-deps --open
```

---

## 💻 Interactive CLI Runner

The root binary provides an interactive command-line interface to inspect algorithms, category metrics, and execute live demonstrations:

```bash
# Run live demonstrations of flagship algorithms (Kadane, QuickSort, SegmentTree, DSU, LRU Cache)
cargo run -- demo

# Display all 20 categories with problem counts and algorithmic highlights
cargo run -- categories

# Show CLI documentation and common workflow shortcuts
cargo run -- help
```

---

## 🏛️ Advanced Data Structures (`src/common`)

Foundational, zero-cost abstractions implemented in 100% safe Rust and parameterized over generic types:

| Data Structure | Module | Time Complexity | Space | Features |
| :--- | :--- | :---: | :---: | :--- |
| **`DisjointSetUnion` (DSU)** | [`src/common/dsu.rs`](src/common/dsu.rs) | $O(\alpha(N))$ | $O(N)$ | Path compression & union-by-rank |
| **`SegmentTree<T, F>`** | [`src/common/segment_tree.rs`](src/common/segment_tree.rs) | $O(\log N)$ query/update | $O(N)$ | Generic associative binary operators |
| **`FenwickTree` (BIT)** | [`src/common/fenwick.rs`](src/common/fenwick.rs) | $O(\log N)$ range/add | $O(N)$ | Fast prefix and range sum queries |
| **`LruCache<K, V>`** | [`src/common/lru_cache.rs`](src/common/lru_cache.rs) | $O(1)$ get/put | $O(C)$ | Safe arena-backed doubly linked list |
| **`ListNode<T>`** | [`src/common/list_node.rs`](src/common/list_node.rs) | $O(1)$ ops | $O(N)$ | Generic single-ownership linked list |
| **`TreeNode<T>`** | [`src/common/tree_node.rs`](src/common/tree_node.rs) | $O(1)$ ops | $O(N)$ | Safe `Rc<RefCell<...>>` binary tree |
| **`Graph` & `Edge`** | [`src/common/graph.rs`](src/common/graph.rs) | $O(1)$ edge add | $O(V + E)$ | Directed & undirected adjacency representation |

---

## 📊 Empirical Benchmarks

The repository includes a dedicated benchmark harness in [`benches/dsa_benchmarks.rs`](benches/dsa_benchmarks.rs) measuring algorithmic throughput and latency:

```bash
cargo bench
```

### Benchmark Highlights (x86_64 Linux, Rust 1.98.1):

- **Sorting (N = 1,000 elements)**:
  - `QuickSort`: **17.0 µs / run**
  - `MergeSort`: **37.7 µs / run**
  - `HeapSort`: **35.3 µs / run**
  - `InsertionSort`: **125.1 µs / run**
- **Disjoint Set Union (DSU)**: 100,000 union and find operations in **683 µs**
- **LRU Cache**: 100,000 get and put operations with eviction in **4.90 ms**
- **Kadane's Algorithm**: 100,000-element array maximum subarray sum in **78.3 µs**
- **Segment Tree**: $O(\log N)$ range sum queries executed across 10,000 requests in **1.13 ms**

---

## 📚 Data Structures & Algorithms Library (395 Problems)

The library provides modular, zero-dependency implementations of classical and advanced DSA problems with integrated unit tests:

| Category | Problems | Description | Directory |
| :--- | :---: | :--- | :--- |
| **Arrays** | 35 | Two-pointer, Dutch National Flag, Kadane, Sliding Window, Boyer-Moore | [1_array](1_array/README.md) |
| **Matrix** | 10 | Spiral traversal, matrix binary search, 2D Kadane, rotations | [2_matrix](2_matrix/README.md) |
| **Strings** | 35 | KMP, Rabin-Karp, Boyer-Moore, Palindromes, Keypad mappings | [3_string](3_string/README.md) |
| **Searching & Sorting** | 26 | Binary search variants, pivot search, inversion count, intervals | [4_search_sort](4_search_sort/README.md) |
| **Linked Lists** | 29 | Singly/doubly/circular lists, Floyd's cycle detection, merge, reverse | [5_linklist](5_linklist/README.md) |
| **Binary Trees** | 35 | Level-order, boundary/diagonal views, LCA, tree reconstruction | [6_binary_tree](6_binary_tree/README.md) |
| **Binary Search Trees** | 22 | Search, insertion, deletion, balancing, LCA, dead ends | [7_bst](7_bst/README.md) |
| **Greedy** | 27 | Activity selection, Huffman coding, fractional knapsack, job sequencing | [8_greedy](8_greedy/README.md) |
| **Backtracking** | 16 | N-Queens, Sudoku solver, Rat in a Maze, M-Coloring, permutations | [9_backtracking](9_backtracking/README.md) |
| **Stacks & Queues** | 15 | Monotonic stack, next greater element, min stack O(1), deque | [10_stack_queues](10_stack_queues/README.md) |
| **Heaps** | 18 | Min/Max heap, running median in stream, K-way merge, heap sort | [11_heap](11_heap/README.md) |
| **Graphs** | 17 | Kahn's topo sort, Dijkstra, Prim/Kruskal MST, cycle detection | [12_graph](12_graph/README.md) |
| **Trie** | 6 | Trie prefix search, shortest unique prefix, word break, phone directory | [13_Trie](13_Trie/README.md) |
| **Dynamic Programming** | 50 | 0/1 & unbounded knapsack, LCS, LIS (O(N log N)), MCM, Catalan, Egg drop | [14_dynamic_programming](14_dynamic_programming/README.md) |
| **Bit Manipulation** | 10 | Count set bits, non-repeating numbers, power set, bitwise math | [15_bit_manipulation](15_bit_manipulation/README.md) |
| **Recursion & Backtracking** | 6 | Tower of Hanoi, combinations, recursive knapsack | [recursion_backtracking](recursion_backtracking/) |
| **Sorting Algorithms** | 6 | Generic Bubble, selection, insertion, merge, quick, and heap sort | [Sorting_Algorithms](Sorting_Algorithms/README.md) |
| **Trees (Advanced)** | 10 | AVL tree self-balancing, level sum, max/min heap properties | [tree](tree/) |
| **Graph (Advanced)** | 7 | Floyd-Warshall all-pairs shortest path, graph dictionary | [graph](graph/) |
| **Basic Algorithms & Math** | 14 | Prime test, digit sums, endianness, bit tricks, file operations | [basic_codes](basic_codes/) |

---

## 📖 Using the DSA Library in Rust

You can import and use any data structure or algorithm directly from `dsa_rust`:

```rust
use dsa_rust::array::p07_kadanes_algorithm::max_sub_array_sum;
use dsa_rust::dynamic_programming::p00_coin_change::count_coin_change;
use dsa_rust::graph::p12_dijkstra_algo::dijkstra;
use dsa_rust::common::tree_node::TreeNode;
use dsa_rust::common::dsu::DisjointSetUnion;

fn main() {
    // Kadane's Algorithm
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(max_sub_array_sum(&arr), 6);

    // Coin Change (ways to make change)
    let coins = [1, 2, 3];
    assert_eq!(count_coin_change(&coins, 4), 4);

    // Tree Construction
    let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
    assert_eq!(TreeNode::to_inorder(&root), vec![2, 1, 3]);

    // Disjoint Set Union
    let mut dsu = DisjointSetUnion::new(5);
    dsu.union(0, 1);
    assert!(dsu.connected(0, 1));
}
```

---

## 📘 Learn Rust Tutorial Chapters

The workspace includes 13 hands-on tutorial projects covering the fundamentals of Rust:

- **[Chapter 1](chapter_1/README.md)**: Hello World, Cargo packages & sections
- **[Chapter 2](chapter_2/README.md)**: Variables, Mutability, Guessing Game
- **[Chapter 3](chapter_3/README.md)**: Reserved keywords and variable shadowing
- **[Chapter 4](chapter_4/)**: Scalar and compound data types (`data_type`)
- **[Chapter 5](chapter_5/)**: Functions, statements vs expressions (`functions`)
- **[Chapter 6](chapter_6/)**: Control flow, `loop`, `while`, `for`, loop labels (`control_flow`)
- **[Chapter 7](chapter_7/)**: Ownership, borrowing, references, and slices (`ownership`)
- **[Chapter 8](chapter_8/)**: Structs, methods, and associated functions (`structs`)
- **[Chapter 9](chapter_9/)**: Enums, `Option<T>`, `match`, and `if let` (`enum_match`)
- **[Chapter 10](chapter_10/)**: Packages, Crates, Modules, and visibility (`package_crates_module`)
- **[Chapter 11](chapter_11/)**: Vectors, HashMaps, Strings, and collections (`data_types`)
- **[Chapter 12](chapter_12/)**: Error handling, `panic!`, `Result<T, E>`, and `?` operator (`panic_results`)
- **[Chapter 13](chapter_13/)**: Generics, Traits, and Lifetimes (`generic_type_traits_lifetimes`)

---

## 🧪 Testing & Verification

All solutions include dedicated unit tests embedded in each file, alongside integration invariant tests and executable doc-tests:

```bash
# Run all workspace unit and integration tests (413 tests)
cargo test --workspace

# Run differential invariant tests
cargo test --test invariants

# Run executable documentation tests (22 doc-tests)
cargo test --doc
```

---

## 📄 License

This repository is maintained for learning and reference purposes. Feel free to use the code for practice, interview preparation, and educational exploration.

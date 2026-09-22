# DSA & Rust Learning Workspace

A comprehensive, production-grade repository featuring **395 Data Structures & Algorithms (DSA)** solutions implemented in idiomatic Rust, alongside a structured 13-chapter **Learn Rust** tutorial suite.

All solutions are written in 100% safe Rust, fully tested with unit tests, properly formatted, and organized within a unified Cargo workspace.

---

## 🚀 Quick Start

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Edition 2021, Rust 1.70+ recommended)

### Build & Test

```bash
# Build the entire workspace
cargo check --workspace

# Run all 396 unit tests
cargo test --workspace

# Run the root binary
cargo run

# Format code and check lints
cargo fmt --check
cargo clippy --workspace

# Generate and view documentation
cargo doc --no-deps --open
```

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
| **Sorting Algorithms** | 6 | Bubble, selection, insertion, merge, quick, and heap sort | [Sorting_Algorithms](Sorting_Algorithms/README.md) |
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

## 🛠️ Project Structure

```text
dsa_rust/
├── Cargo.toml               # Workspace manifest configuration
├── src/
│   ├── main.rs              # Workspace binary entry point
│   ├── lib.rs               # Library root (registers all 20 modules)
│   └── common/              # Shared ListNode, TreeNode, and Graph definitions
├── 1_array/ ... 15_bit_manipulation/  # 20 Category problem suites
└── chapter_1/ ... chapter_13/         # 13 Educational subprojects
```

---

## 🧪 Testing & Verification

All solutions include dedicated unit tests embedded in each file:

```bash
# Run tests for a specific category
cargo test --lib array
cargo test --lib dynamic_programming
cargo test --lib graph

# Run a specific problem test
cargo test p07_kadanes_algorithm
```

---

## 📄 License

This repository is maintained for learning and reference purposes. Feel free to use the code for practice, interview preparation, and educational exploration.

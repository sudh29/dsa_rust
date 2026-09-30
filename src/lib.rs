#![allow(dead_code)]
#![allow(unused_variables)]

pub mod common;

#[path = "../1_array/mod.rs"]
pub mod array;

#[path = "../2_matrix/mod.rs"]
pub mod matrix;

#[path = "../3_string/mod.rs"]
pub mod string;

#[path = "../4_search_sort/mod.rs"]
pub mod search_sort;

#[path = "../5_linklist/mod.rs"]
pub mod linklist;

#[path = "../6_binary_tree/mod.rs"]
pub mod binary_tree;

#[path = "../7_bst/mod.rs"]
pub mod bst;

#[path = "../8_greedy/mod.rs"]
pub mod greedy;

#[path = "../9_backtracking/mod.rs"]
pub mod backtracking;

#[path = "../10_stack_queues/mod.rs"]
pub mod stack_queues;

#[path = "../11_heap/mod.rs"]
pub mod heap;

#[path = "../15_bit_manipulation/mod.rs"]
pub mod bit_manipulation;

pub mod sorting_algorithms {
    pub use crate::search_sort::p34_bubble_sort as p00_bubble_sort;
    pub use crate::search_sort::p35_selection_sort as p01_selection_sort;
    pub use crate::search_sort::p36_insertion_sort as p02_insertion_sort;
    pub use crate::search_sort::p37_merge_sort as p03_merge_sort;
    pub use crate::search_sort::p38_quick_sort as p04_quick_sort;
    pub use crate::search_sort::p39_heap_sort as p05_heap_sort;
}

#[path = "../14_dynamic_programming/mod.rs"]
pub mod dynamic_programming;

#[path = "../12_graph/mod.rs"]
pub mod graph;

#[path = "../13_Trie/mod.rs"]
pub mod trie;

#[path = "../basic_codes/mod.rs"]
pub mod basic_codes;
